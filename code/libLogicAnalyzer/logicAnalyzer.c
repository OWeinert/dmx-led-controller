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
#include "logicAnalyzer.h"

static void logger(const gchar *log_domain, GLogLevelFlags log_level,
		   const gchar *message, gpointer cb_data)
{
	(void)log_domain;
	(void)cb_data;

	/*
	 * All messages, warnings, errors etc. go to stderr (not stdout)
	 */
	if (log_level & (G_LOG_LEVEL_ERROR | G_LOG_LEVEL_CRITICAL | G_LOG_LEVEL_WARNING)
			|| opt_loglevel > SR_LOG_WARN) {
		fprintf(stderr, "%s\n", message);
		fflush(stderr);
	}

	if (log_level & (G_LOG_LEVEL_ERROR | G_LOG_LEVEL_CRITICAL))
		exit(1);
}



gint opt_loglevel = SR_LOG_WARN; /* Show errors+warnings by default. */

__attribute__((unused)) int runAnalyzer(struct CallbackData* callbackData)
{
    struct srd_session *srd_sess = NULL;
	g_log_set_default_handler(logger, NULL);

    struct sr_context *sr_ctx = NULL;

	/* Set the loglevel (amount of messages to output) for libsigrok. */
	if (sr_log_loglevel_set(opt_loglevel) != SR_OK) {
        g_critical("Failed here");
        goto done;
    }
	if (sr_init(&sr_ctx) != SR_OK) {
        g_critical("Failed here");
        goto done;
    }
	/* Set the loglevel (amount of messages to output) for libsigrokdecode. */
	if (srd_log_loglevel_set(opt_loglevel) != SRD_OK) {
        g_critical("Failed here");
        goto done;
    }
    if (srd_init(NULL) != SRD_OK) {
        g_critical("Failed here");
        goto done;
    }
    if (srd_session_new(&srd_sess) != SRD_OK) {
        g_critical("Failed to create new decode session.");
        goto done;
    }
    if (srd_pd_output_callback_add(srd_sess, SRD_OUTPUT_ANN,
                                   srd_callback, callbackData) != SRD_OK) {
        g_critical("Failed here");
        goto done;
    }
    GSList *devices = device_scan(sr_ctx);
    if (!devices) {
        g_critical("No devices found.");
        return -1;
    }
    struct sr_dev_inst *mySaleaeLogic;
    mySaleaeLogic = NULL;

    struct sr_dev_driver *driver;
    for (GSList *sd = devices; sd; sd = sd->next) {
        struct sr_dev_inst *sdi;
        sdi = sd->data;
        driver = sr_dev_inst_driver_get(sdi);

        GArray *drv_opts;
        if (!(drv_opts = sr_dev_options(driver, NULL, NULL))) {
            g_critical("Failed to query list of driver options.");
            return -1;
        }
        for (guint i = 0; i < drv_opts->len; i++) {
            if (g_array_index(drv_opts, uint32_t, i) == SR_CONF_DEMO_DEV) {
                // is demo device
            }

        }
        g_array_free(drv_opts, TRUE);

        if(g_str_equal(driver->name, "fx2lafw") ) {
            mySaleaeLogic = sdi;
            break;
        }
    }
    g_slist_free(devices);

    if (mySaleaeLogic == NULL) {
        g_critical("No real devices found.");
        return -1;
    }

    if (sr_dev_open(mySaleaeLogic) != SR_OK) {
        g_critical("Failed to open device.");
        return -1;
    }

    if (set_dev_options(mySaleaeLogic) != SR_OK) {
        g_critical("Failed here");
        return -1;
    }

    if (srd_decoder_load("dmx512") != SRD_OK) {
        g_critical("Failed to load protocol decoder %s.", "dmx512");
        return SRD_ERR;
    }
    struct srd_decoder_inst *di_H;
    if (!(di_H = srd_inst_new(srd_sess, "dmx512", NULL))) {
        g_critical("Failed to instantiate protocol decoder %s.", "dmx512");
        return SRD_ERR;
    }
    struct srd_decoder_inst *di = srd_inst_find_by_id(srd_sess, di_H->inst_id);
    if (!di) {
        g_critical("Protocol decoder instance \"%s\" not found.", di_H->inst_id);
        return SRD_ERR;
    }


    if (register_pd_with_channels(mySaleaeLogic, di) != SRD_OK) {
        g_critical("Failed here");
        return -1;
    }

    GVariant *gvar;
    if ((sr_dev_config_capabilities_list(mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES) & SR_CONF_LIST)
        && (sr_config_list(driver, mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES, &gvar) == SR_OK))
    {
        /*
         * The device has no compression, or compression is turned
         * off, and publishes its sample memory size.
         */
        uint64_t min_samples, max_samples;
        g_variant_get(gvar, "(tt)", &min_samples, &max_samples);
        g_variant_unref(gvar);
        if (LIMIT_SAMPLES < min_samples) {
            g_critical("The device stores at least %"PRIu64
                               " samples with the current settings.", min_samples);
        }
        if (LIMIT_SAMPLES > max_samples) {
            g_critical("The device can store only %"PRIu64
                               " samples with the current settings.", max_samples);
        }
    }
    gvar = g_variant_new_uint64(LIMIT_SAMPLES);

    if (!(sr_dev_config_capabilities_list(mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES) & SR_CONF_SET)
        || sr_config_set(mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES, gvar) != SR_OK)
    {
        g_critical("Failed to configure sample limit.");
        return -1;
    }

    run_session(mySaleaeLogic, sr_ctx, srd_sess, di);

    srd_exit();
done:
	if (sr_ctx)
		sr_exit(sr_ctx);

	return 0;
}
