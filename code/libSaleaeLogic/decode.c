/*
 * This file is part of the sigrok-cli project.
 *
 * Copyright (C) 2013 Bert Vermeulen <bert@biot.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <stdlib.h>
#include <glib.h>
#include "sigrok-cli.h"

uint64_t pd_samplerate = 0;

extern struct srd_session *srd_sess;

char* channel_id_o;

int register_pd()
{
	struct srd_decoder_inst *di;
	char *pd_name;

	pd_name = "dmx512";

    if (srd_decoder_load(pd_name) != SRD_OK) {
        g_critical("Failed to load protocol decoder %s.", pd_name);
        return 1;
    }

    if (!(di = srd_inst_new(srd_sess, pd_name, NULL))) {
        g_critical("Failed to instantiate protocol decoder %s.", pd_name);
        return 1;
    }

    channel_id_o = g_strdup(di->inst_id);

	return 0;
}


void map_pd_channels(struct sr_dev_inst *sdi)
{
	struct srd_decoder_inst *di = srd_inst_find_by_id(srd_sess, channel_id_o);
	if (!di) {
		g_critical("Protocol decoder instance \"%s\" not found.",
			   (char *)channel_id_o);
		return;
	}
    g_free(channel_id_o);

    char* channelTarget  = "D0";
    GSList *channel_list = sr_dev_inst_channels_get(sdi);
	struct sr_channel *ch = find_channel(channel_list, channelTarget);
    if (!ch) {
        g_printerr("cli: No channel with name \"%s\" found.\n", (char *) channelTarget);
        return;
    }

    if (!ch->enabled)
        g_printerr("cli: Target channel \"%s\" not enabled.\n", (char *) channelTarget);

    GHashTable *channel_indices;
    channel_indices = g_hash_table_new_full(g_str_hash, g_str_equal, g_free,
                                            (GDestroyNotify)g_variant_unref);

    GVariant *var = g_variant_new_int32(ch->index);
    g_variant_ref_sink(var);
    g_hash_table_insert(channel_indices, g_strdup("dmx"), var);
    srd_inst_channel_set_all(di, channel_indices);
    g_hash_table_destroy(channel_indices);
}

/* Convert uint64 sample number to double timestamp in microseconds. */
static double jsontrace_ts_usec(uint64_t snum)
{
	double ts_usec;

	ts_usec = snum;
	ts_usec *= 1e6;
	ts_usec /= pd_samplerate;
	return ts_usec;
}


static void jsontrace_annotation(struct srd_decoder *dec,
	struct srd_proto_data_annotation *pda, struct srd_proto_data *pdata)
{
	char *row_text;
	GSList *lrow, *lcls;
	struct srd_decoder_annotation_row *row;
	int cls;
	char **ann_descr;

	/*
	 * Search for an annotation row for this index, or use the
	 * annotation's descriptor.
	 */
	row_text = NULL;
	if (dec->annotation_rows) {
		for (lrow = dec->annotation_rows; lrow; lrow = lrow->next) {
			row = lrow->data;
			for (lcls = row->ann_classes; lcls; lcls = lcls->next) {
				cls = GPOINTER_TO_INT(lcls->data);
				if (cls == pda->ann_class) {
					row_text = row->desc;
					break;
				}
			}
			if (row_text)
				break;
		}
	}
	if (!row_text) {
		ann_descr = g_slist_nth_data(dec->annotations, pda->ann_class);
		row_text = ann_descr[0];
	}

	printf("\"%s\": \"%s\"", "ph", "B");
	printf("\"%s\": %lf", "ts", jsontrace_ts_usec(pdata->start_sample));
	printf("\"%s\": \"%s\"", "pid", pdata->pdo->proto_id);
	printf("\"%s\": \"%s\"", "tid", row_text);
	printf("\"%s\": \"%s\"\n", "name", pda->ann_text[0]);

	printf("\"%s\": \"%s\"", "ph", "E");
	printf("\"%s\": %lf", "ts", jsontrace_ts_usec(pdata->end_sample));
	printf("\"%s\": \"%s\"", "pid", pdata->pdo->proto_id);
	printf("\"%s\": \"%s\"", "tid", row_text);
	printf("\"%s\": \"%s\"\n", "name", pda->ann_text[0]);
}

void show_pd_annotations(struct srd_proto_data *pdata, void *cb_data)
{
	struct srd_decoder *dec;
	struct srd_proto_data_annotation *pda;

	(void)cb_data;

	dec = pdata->pdo->di->decoder;
	pda = pdata->data;

	/* Google Trace Events are rather special. Use a separate code path. */
    jsontrace_annotation(dec, pda, pdata);
}

