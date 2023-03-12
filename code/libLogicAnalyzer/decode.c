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

#include <glib.h>
#include "logicAnalyzer.h"

int register_pd_with_channels(struct sr_dev_inst *sdi, struct srd_decoder_inst *di)
{
    GSList *channel_list = sr_dev_inst_channels_get(sdi);
    struct sr_channel *ch = g_slist_nth(channel_list, 0)->data;
    if (strcmp(ch->name, "D0") != 0) {
        g_printerr("cli: No channel with name \"%s\" found.\n", (char *) "D0");
        return SRD_ERR;
    }
    if (!ch->enabled) {
        g_printerr("cli: Target channel \"%s\" not enabled.\n", (char *) "D0");
        return SRD_ERR;
    }
    GHashTable *channel_indices = g_hash_table_new_full(
        g_str_hash,
        g_str_equal,
        g_free,
        (GDestroyNotify)g_variant_unref
    );
    g_hash_table_insert(
        channel_indices,
        g_strdup("dmx"),
        g_variant_ref_sink(g_variant_new_int32(ch->index))
    );
    srd_inst_channel_set_all(di, channel_indices);
    g_hash_table_destroy(channel_indices);
    return SRD_OK;
}

void srd_callback(struct srd_proto_data *pdata, void *cb_data)
{
    struct CallbackData *rustCall = cb_data;
    rustCall->onDecoderAnnotation(rustCall->rustData, pdata);
}
