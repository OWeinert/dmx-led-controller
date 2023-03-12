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
#include <stdlib.h>
#include "logicAnalyzer.h"

void sr_session_callback(const struct sr_dev_inst *sdi, const struct sr_datafeed_packet *packet, void *cb_data)
{
	static uint64_t received_samples_logic = 0;

    /* Skip all packets before the first header. */
	if (packet->type != SR_DF_HEADER && !((struct cb_data *) cb_data)->receivingEnabled) {
        return;
    }
	switch (packet->type) {
	case SR_DF_HEADER: {
		g_debug("cli: Received SR_DF_HEADER.");
        ((struct cb_data *) cb_data)->receivingEnabled = TRUE;
        received_samples_logic = 0;
		break;
    }
	case SR_DF_META:
		g_debug("cli: Received SR_DF_META.");
		break;

	case SR_DF_TRIGGER:
		g_debug("cli: Received SR_DF_TRIGGER.");
		break;

	case SR_DF_LOGIC: {
        const struct sr_datafeed_logic *logic = packet->payload;
		g_message("cli: Received SR_DF_LOGIC (%"PRIu64" bytes, unitsize = %d).",
				logic->length, logic->unitsize);
		if (logic->length == 0)
			break;

		if (received_samples_logic >= LIMIT_SAMPLES)
			break;
        uint64_t end_sample = received_samples_logic + logic->length / logic->unitsize;
		/* Cut off last packet according to the sample limit. */
		if (end_sample > LIMIT_SAMPLES != 0)
			end_sample = LIMIT_SAMPLES;
        uint64_t input_len = (end_sample - received_samples_logic) * logic->unitsize;
        struct srd_session *srd_sess = ((struct cb_data *) cb_data)->srd_session;
        if (srd_session_send(srd_sess, received_samples_logic, end_sample,
                             logic->data, input_len, logic->unitsize) != SRD_OK) {
                    sr_session_stop(((struct cb_data *) cb_data)->sr_session);
        }
        struct srd_decoder_inst* di = ((struct cb_data *) cb_data)->di;
        g_mutex_lock (&di->data_mutex);
        while (!di->handled_all_samples)
            g_cond_wait (&di->handled_all_samples_cond, &di->data_mutex);
        g_mutex_unlock (&di->data_mutex);

        received_samples_logic = end_sample;
		break;
    }
	case SR_DF_ANALOG: {
        const struct sr_datafeed_analog *analog = packet->payload;
        g_message("cli: Received SR_DF_ANALOG (%d samples).", analog->num_samples);
        break;
    }
	case SR_DF_FRAME_BEGIN: {
        g_debug("cli: Received SR_DF_FRAME_BEGIN.");
        break;
    }
	case SR_DF_FRAME_END:
		g_debug("cli: Received SR_DF_FRAME_END.");
		break;

    case SR_DF_END:
        g_debug("cli: Received SR_DF_END.");
        struct srd_decoder_inst *di = ((struct cb_data *) cb_data)->di;
        // TODO thats actually just bullshit...
        int hmmmmm = 0;
        while(!hmmmmm) {
            hmmmmm = di->handled_all_samples && (di->abs_cur_samplenum == LIMIT_SAMPLES);
        }

        ((struct cb_data *) cb_data)->receivingEnabled = FALSE;
        if (received_samples_logic > 0 && received_samples_logic < LIMIT_SAMPLES)
            g_warning("Device only sent %" PRIu64 " samples.",
                      received_samples_logic);
        break;

	default:
		break;
	}
}

struct cb_data_test {
    GMainLoop *loop;

    struct srd_decoder_inst *di;
};

void test(void *data) {
    struct srd_decoder_inst *di = ((struct cb_data_test *) data)->di;
    g_mutex_lock (&di->data_mutex);
    while (!di->handled_all_samples)
        g_cond_wait (&di->handled_all_samples_cond, &di->data_mutex);
    g_mutex_unlock (&di->data_mutex);

    struct cb_data_test *realData = (struct cb_data_test *) data;

    GMainLoop *loop = realData->loop;
    g_main_loop_quit(loop);
}

void run_session(struct sr_dev_inst *mySaleaeLogic, struct sr_context *sr_ctx, struct srd_session *srd_session, struct srd_decoder_inst *di)
{
    struct sr_session *sr_session;
	sr_session_new(sr_ctx, &sr_session);
	struct cb_data cb_data = {
            srd_session,
            sr_session,
            FALSE,
            di,
    };
    sr_session_datafeed_callback_add(sr_session, sr_session_callback, &cb_data);


    if (sr_session_dev_add(sr_session, mySaleaeLogic) != SR_OK) {
        g_critical("Failed to add device to sr_session.");
        sr_session_destroy(sr_session);
        return;
    }
    GMainLoop *main_loop;
    main_loop = g_main_loop_new(NULL, FALSE);
    struct cb_data_test callback_data_test = {
        main_loop,
        di,

    };
    sr_session_stopped_callback_set(sr_session,
                                    test, &callback_data_test);

    if (sr_session_start(sr_session) != SR_OK) {
        g_critical("Failed to start sr_session.");
        g_main_loop_unref(main_loop);
        sr_session_destroy(sr_session);
        return;
    }

    uint64_t samplerate = 0;
    if (sr_dev_config_capabilities_list(mySaleaeLogic, NULL, SR_CONF_SAMPLERATE) & SR_CONF_GET) {
        GVariant *gvar;
        struct sr_dev_driver *driver = sr_dev_inst_driver_get(mySaleaeLogic);
        if (sr_config_get(driver, mySaleaeLogic, NULL, SR_CONF_SAMPLERATE, &gvar) == SR_OK) {
            samplerate = g_variant_get_uint64(gvar);
            g_variant_unref(gvar);
        }
    }
    if (samplerate == 0) {
        g_critical("No samplerate defined, cant start decoder");
        return;
    }

    if (srd_session_metadata_set(srd_session, SRD_CONF_SAMPLERATE,
                                 g_variant_new_uint64(samplerate)) != SRD_OK) {
        g_critical("Failed to configure decode session.");
        return;
    }
    if (srd_session_start(srd_session) != SRD_OK) {
        g_critical("Failed to start decode session.");
        return;
    }


	g_main_loop_run(main_loop);

    g_mutex_lock (&di->data_mutex);
    while (!di->handled_all_samples)
        g_cond_wait (&di->handled_all_samples_cond, &di->data_mutex);
    g_mutex_unlock (&di->data_mutex);
    //sr_session_stop(sr_session);
	//sr_session_destroy(sr_session);

    //while(1);

	//sr_session_datafeed_callback_remove_all(sr_session);
	g_main_loop_unref(main_loop);
    srd_session_destroy(srd_session);

}
