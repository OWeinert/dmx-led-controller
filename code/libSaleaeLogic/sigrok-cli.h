/*
 * This file is part of the sigrok-cli project.
 *
 * Copyright (C) 2011 Bert Vermeulen <bert@biot.com>
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

#ifndef SIGROK_CLI_SIGROK_CLI_H
#define SIGROK_CLI_SIGROK_CLI_H

#include <libsigrokdecode/libsigrokdecode.h>
#include <libsigrok/libsigrok.h>

#define DEFAULT_OUTPUT_FORMAT_FILE "srzip"
#define DEFAULT_OUTPUT_FORMAT_NOFILE "bits:width=64"

/* main.c */
extern gint opt_loglevel;
extern gchar *opt_output_file;
extern gchar *opt_drv;
extern gchar *opt_output_format;
int mainC();
extern struct sr_context *sr_ctx;
int select_channels(struct sr_dev_inst *sdi);
int maybe_config_get(struct sr_dev_driver *driver,
		const struct sr_dev_inst *sdi, struct sr_channel_group *cg,
		uint32_t key, GVariant **gvar);
int maybe_config_set(struct sr_dev_driver *driver,
		const struct sr_dev_inst *sdi, struct sr_channel_group *cg,
		uint32_t key, GVariant *gvar);
int maybe_config_list(struct sr_dev_driver *driver,
		const struct sr_dev_inst *sdi, struct sr_channel_group *cg,
		uint32_t key, GVariant **gvar);


/* device.c */
GSList *device_scan(void);

/* session.c */
struct df_arg_desc {
	struct sr_session *session;
	int do_props;
	struct input_stream_props {
		uint64_t samplerate;
		GSList *channels;
		const struct sr_channel *first_analog_channel;
		size_t unitsize;
		uint64_t sample_count_logic;
		uint64_t sample_count_analog;
		uint64_t frame_count;
		uint64_t triggered;
	} props;
};
void datafeed_in(const struct sr_dev_inst *sdi,
		const struct sr_datafeed_packet *packet, void *cb_data);
int opt_to_gvar(struct sr_config *src);
void run_session(void);

/* output.c */
int setup_binary_stdout(void);

/* decode.c */
extern uint64_t pd_samplerate;
int register_pd();
void show_pd_annotations(struct srd_proto_data *pdata, void *cb_data);
void map_pd_channels(struct sr_dev_inst *sdi);

/* parsers.c */
struct sr_channel *find_channel(GSList *channellist, const char *channelname);
GHashTable *parse_generic_arg(const char *arg,
		gboolean sep_first, const char *key_first);
GHashTable *generic_arg_to_opt(const struct sr_option **opts, GHashTable *genargs);
GSList *check_unknown_keys(const struct sr_option **avail, GHashTable *used);
gboolean warn_unknown_keys(const struct sr_option **avail, GHashTable *used,
		const char *caption);
int get_driver(struct sr_dev_driver **driver, GSList **drvopts);

#endif
