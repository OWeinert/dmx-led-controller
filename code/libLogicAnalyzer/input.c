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


void load_input_file(struct sr_context *sr_ctx, struct srd_session *srd_session, uint64_t sampleRate)
{
    struct sr_session *session;
    if (sr_session_load(sr_ctx, "assets/sigrok_sessions/colored_spacecraft_24MHz.sr", &session) == SR_OK) {
        /* sigrok session file */
        GSList *devices;
        if (sr_session_dev_list(session, &devices) != SR_OK || !devices || !devices->data) {
            g_critical("Failed to access session device.");
            g_slist_free(devices);
            sr_session_destroy(session);
            return;
        }
        g_slist_free(devices);
        GMainLoop *main_loop;
        main_loop = g_main_loop_new(NULL, FALSE);

        struct cb_data cb_data = {
                srd_session,
                session,
                LIMIT_SAMPLES(sampleRate),
        };

        sr_session_datafeed_callback_add(session,sr_session_callback, &cb_data);
        sr_session_stopped_callback_set(session,
                                        (sr_session_stopped_callback)g_main_loop_quit,
                                        main_loop);
        if (sr_session_start(session) == SR_OK)
            g_main_loop_run(main_loop);

        g_main_loop_unref(main_loop);
        sr_session_destroy(session);
    }
}
