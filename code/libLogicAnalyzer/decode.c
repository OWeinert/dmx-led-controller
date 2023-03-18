/*
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
#include "libLogicAnalyzer.h"

void srd_callback(struct srd_proto_data *pdata, void *cb_data);

int register_pd_with_channels(struct sr_dev_inst *sdi, struct srd_decoder_inst *di)
{
    GSList *channel_list = sr_dev_inst_channels_get(sdi);
    struct sr_channel *ch = g_slist_nth(channel_list, CHANNEL)->data;
    if (!ch->enabled) {
        g_printerr("logic analyzer: Target channel \"%d\" not enabled.\n", CHANNEL);
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
            g_variant_ref_sink(g_variant_new_int32(CHANNEL))
    );
    srd_inst_channel_set_all(di, channel_indices);
    g_hash_table_destroy(channel_indices);
    return SRD_OK;
}

int sigrok_decode_session_start(struct srd_session **srd_sess, struct CallbackData* callbackData, gint opt_loglevel, struct srd_decoder_inst **di, struct sr_dev_inst *sdi) {
    /* Set the loglevel (amount of messages to output) for libsigrokdecode. */
    if (srd_log_loglevel_set(opt_loglevel) != SRD_OK) {
        g_critical("logic analyzer: Error 200 occurred, exiting");
        return SR_ERR;
    }
    if (srd_init(NULL) != SRD_OK) {
        g_critical("logic analyzer: Error 201 occurred, exiting");
        return SR_ERR;
    }
    if (srd_session_new(srd_sess) != SRD_OK) {
        g_critical("Failed to create new decode session.");
        return SR_ERR;
    }
    if (srd_pd_output_callback_add(*srd_sess, SRD_OUTPUT_ANN,
                                   srd_callback, callbackData) != SRD_OK) {
        g_critical("logic analyzer: Error 202 occurred, exiting");
        return SR_ERR;
    }
    if (srd_decoder_load(PROTOCOL_DECODER) != SRD_OK) {
        g_critical("Failed to load protocol decoder %s.", PROTOCOL_DECODER);
        return SRD_ERR;
    }
    struct srd_decoder_inst *di_H = NULL;
    if (!(di_H = srd_inst_new(*srd_sess, PROTOCOL_DECODER, NULL))) {
        g_critical("Failed to instantiate protocol decoder %s.", PROTOCOL_DECODER);
        return SRD_ERR;
    }
    *di = srd_inst_find_by_id(*srd_sess, di_H->inst_id);
    if (!*di) {
        g_critical("Protocol decoder instance \"%s\" not found.", di_H->inst_id);
        return SRD_ERR;
    }

    if (srd_session_metadata_set(*srd_sess, SRD_CONF_SAMPLERATE,g_variant_new_uint64(SAMPLE_RATE)) != SRD_OK) {
        g_critical("Failed to configure decode session.");
        return SRD_ERR;
    }

    if (register_pd_with_channels(sdi, *di) != SRD_OK) {
        g_critical("logic analyzer: Error 203 occurred, exiting");
        return SRD_ERR;
    }
    if (srd_session_start(*srd_sess) != SRD_OK) {
        g_critical("Failed to start decode session.");
        return SRD_ERR;
    }

    return SRD_OK;
}

void srd_callback(struct srd_proto_data *pdata, void *cb_data)
{
    struct CallbackData *rustCall = cb_data;
    rustCall->onDecoderAnnotation(rustCall->rustData, pdata);
}
