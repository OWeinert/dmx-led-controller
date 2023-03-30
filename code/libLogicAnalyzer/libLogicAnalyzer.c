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

#include <stdlib.h>
#include <glib.h>
#include <stdbool.h>
#include "libLogicAnalyzer.h"

static void logger(
        const gchar *log_domain,
        GLogLevelFlags log_level,
        const gchar *message,
        gpointer cb_data
) {
	(void)log_domain;
    gint* opt_loglevel = (gint*) cb_data;

	/*
	 * All messages, warnings, errors etc. go to stderr (not stdout)
	 */
	if (log_level & (G_LOG_LEVEL_ERROR | G_LOG_LEVEL_CRITICAL | G_LOG_LEVEL_WARNING)
			|| *opt_loglevel > SR_LOG_WARN) {
		fprintf(stderr, "%s\n", message);
		fflush(stderr);
	}

	if (log_level & (G_LOG_LEVEL_ERROR | G_LOG_LEVEL_CRITICAL))
		exit(1);
}

__attribute__((unused)) int runAnalyzer(struct CallbackData* callbackData, bool fromDevice, uint64_t sampleRate)
{
    gint opt_loglevel = SR_LOG_WARN; /* Show errors+warnings by default. */
	g_log_set_default_handler(logger, &opt_loglevel);

    struct sr_context *sr_ctx = NULL;

	/* Set the loglevel (amount of messages to output) for libsigrok. */
	if (sr_log_loglevel_set(opt_loglevel) != SR_OK) {
        g_critical("logic analyzer: Error 100 occurred, exiting");
        goto done;
    }

	if (sr_init(&sr_ctx) != SR_OK) {
        g_critical("logic analyzer: Error 101 occurred, exiting");
        goto done;
    }
    struct sr_dev_inst *mySaleaeLogic = NULL;

    if (fromDevice && device_init(&mySaleaeLogic, sr_ctx, sampleRate) != SR_OK) {
        g_critical("logic analyzer: Error 102 occurred, exiting");
        goto done;
    }

    struct srd_session *srd_session = NULL;
    struct srd_decoder_inst *decoder_instant = NULL;
    if (sigrok_decode_session_start(&srd_session, callbackData, opt_loglevel, &decoder_instant, mySaleaeLogic, sampleRate) != SR_OK) {
        g_critical("logic analyzer: Error 103 occurred, exiting");
        goto done;
    }
    if (fromDevice) {
        run_session(mySaleaeLogic, sr_ctx, srd_session, sampleRate);
    } else {
        load_input_file(sr_ctx, srd_session, sampleRate);
    }

    g_mutex_lock (&decoder_instant->data_mutex);
    while (!decoder_instant->handled_all_samples)
        g_cond_wait (&decoder_instant->handled_all_samples_cond, &decoder_instant->data_mutex);
    g_mutex_unlock (&decoder_instant->data_mutex);

    srd_exit();

    done:
    if (sr_ctx)
        sr_exit(sr_ctx);

    return 0;
}
