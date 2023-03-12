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

struct sr_context *sr_ctx = NULL;
struct srd_session *srd_sess = NULL;

static void logger(const gchar *log_domain, GLogLevelFlags log_level,
		   const gchar *message, gpointer cb_data)
{
	(void)log_domain;
	(void)cb_data;

	/*
	 * All messages, warnings, errors etc. go to stderr (not stdout) in
	 * order to not mess up the CLI tool data output, e.g. VCD output.
	 */
	if (log_level & (G_LOG_LEVEL_ERROR | G_LOG_LEVEL_CRITICAL | G_LOG_LEVEL_WARNING)
			|| opt_loglevel > SR_LOG_WARN) {
		fprintf(stderr, "%s\n", message);
		fflush(stderr);
	}

	if (log_level & (G_LOG_LEVEL_ERROR | G_LOG_LEVEL_CRITICAL))
		exit(1);

}

int select_channels(struct sr_dev_inst *sdi)
{
	sr_dev_inst_channels_get(sdi);
    map_pd_channels(sdi);
	return SR_OK;
}

int maybe_config_get(struct sr_dev_driver *driver,
		const struct sr_dev_inst *sdi, struct sr_channel_group *cg,
		uint32_t key, GVariant **gvar)
{
	if (sr_dev_config_capabilities_list(sdi, cg, key) & SR_CONF_GET)
		return sr_config_get(driver, sdi, cg, key, gvar);

	return SR_ERR_NA;
}

int maybe_config_set(struct sr_dev_driver *driver,
		const struct sr_dev_inst *sdi, struct sr_channel_group *cg,
		uint32_t key, GVariant *gvar)
{
	(void)driver;

	if (sr_dev_config_capabilities_list(sdi, cg, key) & SR_CONF_SET)
		return sr_config_set(sdi, cg, key, gvar);

	return SR_ERR_NA;
}

int maybe_config_list(struct sr_dev_driver *driver,
		const struct sr_dev_inst *sdi, struct sr_channel_group *cg,
		uint32_t key, GVariant **gvar)
{
	if (sr_dev_config_capabilities_list(sdi, cg, key) & SR_CONF_LIST)
		return sr_config_list(driver, sdi, cg, key, gvar);

	return SR_ERR_NA;
}

/*
 * sigrok-cli
 *  -d fx2lafw
 *  --samples 700
 *  -P dmx512:dmx=D0
 *  --config "samplerate=1 MHz"
 *  --protocol-decoder-jsontrace
 */

gint opt_loglevel = SR_LOG_WARN; /* Show errors+warnings by default. */
gchar *opt_output_file = NULL;
gchar *opt_drv = "fx2lafw";
gchar *opt_output_format = NULL;

int mainC()
{
	g_log_set_default_handler(logger, NULL);

	/* Set the loglevel (amount of messages to output) for libsigrok. */
	if (sr_log_loglevel_set(opt_loglevel) != SR_OK)
		goto done;

	if (sr_init(&sr_ctx) != SR_OK)
		goto done;

	/* Set the loglevel (amount of messages to output) for libsigrokdecode. */
	if (srd_log_loglevel_set(opt_loglevel) != SRD_OK)
		goto done;

    if (srd_init(NULL) != SRD_OK)
        goto done;
    if (srd_session_new(&srd_sess) != SRD_OK) {
        g_critical("Failed to create new decode session.");
        goto done;
    }
    if (register_pd() != 0)
        goto done;

    if (srd_pd_output_callback_add(srd_sess, SRD_OUTPUT_ANN,
            show_pd_annotations, NULL) != SRD_OK)
        goto done;

    run_session();
    srd_exit();

done:
	if (sr_ctx)
		sr_exit(sr_ctx);

	return 0;
}
