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
#include <stdlib.h>
#include "libLogicAnalyzer.h"

struct cb_data {
    struct srd_session *srd_session;
    struct sr_session *sr_session;
};

void sr_session_callback(const struct sr_dev_inst *sdi, const struct sr_datafeed_packet *packet, void *cb_data)
{
	static uint64_t received_samples = 0;
	switch (packet->type) {
	case SR_DF_HEADER: {
		g_debug("logic analyzer: Received SR_DF_HEADER.");
        received_samples = 0;
		break;
    }
	case SR_DF_LOGIC: {
        const struct sr_datafeed_logic *logic = packet->payload;
		g_message("logic analyzer: Received SR_DF_LOGIC (%"PRIu64" bytes, unitsize = %d).", logic->length, logic->unitsize);
		if (logic->length == 0)
			break;
		if (received_samples >= LIMIT_SAMPLES) {
            g_critical("receiving to many samples");
            break;
        }
        uint64_t end_sample = received_samples + (logic->length / logic->unitsize);
        struct srd_session *srd_sess = ((struct cb_data *) cb_data)->srd_session;
        if (srd_session_send(srd_sess, received_samples, end_sample,
                             logic->data, logic->length, logic->unitsize) != SRD_OK) {
                    sr_session_stop(((struct cb_data *) cb_data)->sr_session);
        }
        received_samples = end_sample;
		break;
    }
    case SR_DF_END:
        g_debug("logic analyzer: Received SR_DF_END.");
        if (received_samples != LIMIT_SAMPLES)
            g_warning("Device only sent %" PRIu64 " samples.", received_samples);
        break;

	default:
        g_warning("logic analyzer: Received unhandled package type.");
		break;
	}
}

struct sr_session_stop_callback_data {
    GMainLoop *loop;
};

void on_sr_session_stopped(void *data) {
    GMainLoop *loop = ((struct sr_session_stop_callback_data *) data)->loop;
    g_main_loop_quit(loop);
}

void run_session(struct sr_dev_inst *mySaleaeLogic, struct sr_context *sr_ctx, struct srd_session *srd_session)
{
    struct sr_session *sr_session;
	sr_session_new(sr_ctx, &sr_session);
	struct cb_data cb_data = {
            srd_session,
            sr_session,
    };
    sr_session_datafeed_callback_add(sr_session, sr_session_callback, &cb_data);

    if (sr_session_dev_add(sr_session, mySaleaeLogic) != SR_OK) {
        g_critical("Failed to add device to sr_session.");
        sr_session_destroy(sr_session);
        return;
    }
    GMainLoop *main_loop;
    main_loop = g_main_loop_new(NULL, FALSE);
    struct sr_session_stop_callback_data callback_data_test = {
        main_loop,
    };
    sr_session_stopped_callback_set(sr_session,on_sr_session_stopped, &callback_data_test);

    if (sr_session_start(sr_session) != SR_OK) {
        g_critical("Failed to start sr_session.");
        g_main_loop_unref(main_loop);
        sr_session_destroy(sr_session);
        return;
    }
	g_main_loop_run(main_loop);
	g_main_loop_unref(main_loop);
	sr_session_destroy(sr_session);
}
