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
#include <glib/gstdio.h>
#include <string.h>
#include <stdlib.h>
#include "sigrok-cli.h"

static uint64_t limit_samples = 700;

extern struct srd_session *srd_sess;

const struct sr_output *setup_output_format(const struct sr_dev_inst *sdi, FILE **outfile)
{
	const struct sr_output_module *omod;
	const struct sr_option **options;
	const struct sr_output *o;
	GHashTable *fmtargs, *fmtopts;
	char *fmtspec;

	if (!opt_output_format) {
		if (opt_output_file) {
			opt_output_format = DEFAULT_OUTPUT_FORMAT_FILE;
		} else {
			opt_output_format = DEFAULT_OUTPUT_FORMAT_NOFILE;
		}
	}

	fmtargs = parse_generic_arg(opt_output_format, TRUE, NULL);
	fmtspec = g_hash_table_lookup(fmtargs, "sigrok_key");
	if (!fmtspec)
		g_critical("Invalid output format.");
	if (!(omod = sr_output_find(fmtspec)))
		g_critical("Unknown output module '%s'.", fmtspec);
	g_hash_table_remove(fmtargs, "sigrok_key");
	if ((options = sr_output_options_get(omod))) {
		fmtopts = generic_arg_to_opt(options, fmtargs);
		(void)warn_unknown_keys(options, fmtargs, NULL);
		sr_output_options_free(options);
	} else {
		fmtopts = NULL;
	}
	o = sr_output_new(omod, fmtopts, sdi, opt_output_file);

	if (opt_output_file) {
		if (!sr_output_test_flag(omod, SR_OUTPUT_INTERNAL_IO_HANDLING)) {
			*outfile = g_fopen(opt_output_file, "wb");
			if (!*outfile) {
				g_critical("Cannot write to output file '%s'.",
					opt_output_file);
			}
		} else {
			*outfile = NULL;
		}
	} else {
		setup_binary_stdout();
		*outfile = stdout;
	}

	if (fmtopts)
		g_hash_table_destroy(fmtopts);
	g_hash_table_destroy(fmtargs);

	return o;
}


/* Get the input stream's list of channels and their types, once. */
static void props_get_channels(struct df_arg_desc *args,
	const struct sr_dev_inst *sdi)
{
	struct input_stream_props *props;
	GSList *l;
	const struct sr_channel *ch;

	if (!args)
		return;
	props = &args->props;
	if (props->channels)
		return;

	props->channels = g_slist_copy(sr_dev_inst_channels_get(sdi));
	if (!props->channels)
		return;
	for (l = props->channels; l; l = l->next) {
		ch = l->data;
		if (!ch->enabled)
			continue;
		if (ch->type != SR_CHANNEL_ANALOG)
			continue;
		props->first_analog_channel = ch;
		break;
	}
}

static void props_dump_details(struct df_arg_desc *args)
{
	struct input_stream_props *props;
	size_t ch_count;
	GSList *l;
	const struct sr_channel *ch;
	const char *type;

	if (!args)
		return;
	props = &args->props;
	if (props->samplerate)
		printf("Samplerate: %" PRIu64 "\n", props->samplerate);
	if (props->channels) {
		ch_count = g_slist_length(props->channels);
		printf("Channels: %zu\n", ch_count);
		for (l = props->channels; l; l = l->next) {
			ch = l->data;
			if (ch->type == SR_CHANNEL_ANALOG)
				type = "analog";
			else
				type = "logic";
			printf("- %s: %s\n", ch->name, type);
		}
	}
	if (props->unitsize)
		printf("Logic unitsize: %zu\n", props->unitsize);
	if (props->sample_count_logic)
		printf("Logic sample count: %" PRIu64 "\n", props->sample_count_logic);
	if (props->sample_count_analog)
		printf("Analog sample count: %" PRIu64 "\n", props->sample_count_analog);
	if (props->frame_count)
		printf("Frame count: %" PRIu64 "\n", props->frame_count);
	if (props->triggered)
		printf("Trigger count: %" PRIu64 "\n", props->triggered);
}

static void props_cleanup(struct df_arg_desc *args)
{
	struct input_stream_props *props;

	if (!args)
		return;
	props = &args->props;
	g_slist_free(props->channels);
	props->channels = NULL;
	props->first_analog_channel = NULL;
}

void datafeed_in(const struct sr_dev_inst *sdi,
		const struct sr_datafeed_packet *packet, void *cb_data)
{
	static const struct sr_output *o = NULL;
	static const struct sr_output *oa = NULL;
	static uint64_t rcvd_samples_logic = 0;
	static uint64_t rcvd_samples_analog = 0;
	static uint64_t samplerate = 0;
	static FILE *outfile = NULL;

	const struct sr_datafeed_meta *meta;
	const struct sr_datafeed_logic *logic;
	const struct sr_datafeed_analog *analog;
	struct df_arg_desc *df_arg;
	int do_props;
	struct input_stream_props *props;
	struct sr_session *session;
	struct sr_config *src;
	GSList *l;
	GVariant *gvar;
	uint64_t end_sample;
	uint64_t input_len;
	struct sr_dev_driver *driver;

	driver = sr_dev_inst_driver_get(sdi);

	/* Skip all packets before the first header. */
	if (packet->type != SR_DF_HEADER && !o)
		return;

	/* Prepare to either process data, or "just" gather properties. */
	df_arg = cb_data;
	session = df_arg->session;
	do_props = df_arg->do_props;
	props = &df_arg->props;

	switch (packet->type) {
	case SR_DF_HEADER:
		g_debug("cli: Received SR_DF_HEADER.");
		if (maybe_config_get(driver, sdi, NULL, SR_CONF_SAMPLERATE,
				&gvar) == SR_OK) {
			samplerate = g_variant_get_uint64(gvar);
			g_variant_unref(gvar);
		}
		if (do_props) {
			/* Setup variables for maximum code path re-use. */
			o = (void *)-1;
			limit_samples = 0;
			/* Start collecting input stream properties. */
			memset(props, 0, sizeof(*props));
			props->samplerate = samplerate;
			props_get_channels(df_arg, sdi);
			break;
		}
		if (!(o = setup_output_format(sdi, &outfile)))
			g_critical("Failed to initialize output module.");

		/* Set up backup analog output module. */
		if (outfile)
			oa = sr_output_new(sr_output_find("analog"), NULL,
					sdi, NULL);

		rcvd_samples_logic = rcvd_samples_analog = 0;

        if (samplerate) {
            if (srd_session_metadata_set(srd_sess, SRD_CONF_SAMPLERATE,
                    g_variant_new_uint64(samplerate)) != SRD_OK) {
                g_critical("Failed to configure decode session.");
                break;
            }
            pd_samplerate = samplerate;
        }
        if (srd_session_start(srd_sess) != SRD_OK) {
            g_critical("Failed to start decode session.");
            break;
        }

		break;

	case SR_DF_META:
		g_debug("cli: Received SR_DF_META.");
		meta = packet->payload;
		for (l = meta->config; l; l = l->next) {
			src = l->data;
			switch (src->key) {
			case SR_CONF_SAMPLERATE:
				samplerate = g_variant_get_uint64(src->data);
				g_debug("cli: Got samplerate %"PRIu64" Hz.", samplerate);
				if (do_props) {
					props->samplerate = samplerate;
					break;
				}

                if (srd_session_metadata_set(srd_sess, SRD_CONF_SAMPLERATE,
                        g_variant_new_uint64(samplerate)) != SRD_OK) {
                    g_critical("Failed to pass samplerate to decoder.");
                }
                pd_samplerate = samplerate;

				break;
			case SR_CONF_SAMPLE_INTERVAL:
				samplerate = g_variant_get_uint64(src->data);
				g_debug("cli: Got sample interval %"PRIu64" ms.", samplerate);
				if (do_props) {
					props->samplerate = samplerate;
					break;
				}
				break;
			default:
				/* Unknown metadata is not an error. */
				break;
			}
		}
		break;

	case SR_DF_TRIGGER:
		g_debug("cli: Received SR_DF_TRIGGER.");
		break;

	case SR_DF_LOGIC:
		logic = packet->payload;
		g_message("cli: Received SR_DF_LOGIC (%"PRIu64" bytes, unitsize = %d).",
				logic->length, logic->unitsize);
		if (logic->length == 0)
			break;

		if (do_props) {
			props_get_channels(df_arg, sdi);
			props->unitsize = logic->unitsize;
			props->sample_count_logic += logic->length / logic->unitsize;
			break;
		}


		if (limit_samples && rcvd_samples_logic >= limit_samples)
			break;

		end_sample = rcvd_samples_logic + logic->length / logic->unitsize;
		/* Cut off last packet according to the sample limit. */
		if (limit_samples && end_sample > limit_samples)
			end_sample = limit_samples;
		input_len = (end_sample - rcvd_samples_logic) * logic->unitsize;


        if (srd_session_send(srd_sess, rcvd_samples_logic, end_sample,
                logic->data, input_len, logic->unitsize) != SRD_OK)
            sr_session_stop(session);

		rcvd_samples_logic = end_sample;
		break;

	case SR_DF_ANALOG:
		analog = packet->payload;
		g_message("cli: Received SR_DF_ANALOG (%d samples).", analog->num_samples);
        break;

	case SR_DF_FRAME_BEGIN:
		g_debug("cli: Received SR_DF_FRAME_BEGIN.");
		break;

	case SR_DF_FRAME_END:
		g_debug("cli: Received SR_DF_FRAME_END.");
		if (do_props) {
			props->frame_count++;
			break;
		}
		break;

	default:
		break;
	}

	/*
	 * SR_DF_END needs to be handled after the output module's receive()
	 * is called, so it can properly clean up that module.
	 */
	if (packet->type == SR_DF_END) {
		g_debug("cli: Received SR_DF_END.");


		if (do_props) {
			props_dump_details(df_arg);
			props_cleanup(df_arg);
			o = NULL;
		}

		if (o)
			sr_output_free(o);
		o = NULL;

		if (oa)
			sr_output_free(oa);
		oa = NULL;

		if (outfile && outfile != stdout)
			fclose(outfile);

		if (limit_samples) {
			if (rcvd_samples_logic > 0 && rcvd_samples_logic < limit_samples)
				g_warning("Device only sent %" PRIu64 " samples.",
					   rcvd_samples_logic);
			else if (rcvd_samples_analog > 0 && rcvd_samples_analog < limit_samples)
				g_warning("Device only sent %" PRIu64 " samples.",
					   rcvd_samples_analog);
		}
	}

}

int set_dev_options(struct sr_dev_inst *sdi)
{
	struct sr_config src;
	struct sr_channel_group *cg;
	int ret;
	cg = NULL;
    const struct sr_key_info *srci;

    srci = sr_key_info_name_get(SR_KEY_CONFIG, "samplerate");
    if (!srci) {
        g_critical("Unknown device option '%s'.", "samplerate");
        return -1;
    }

    src.key = srci->key;
    src.data = g_variant_new_uint64((uint64_t) 1e6);

    if ((ret = maybe_config_set(sr_dev_inst_driver_get(sdi), sdi, cg,
            src.key, src.data)) != SR_OK) {
        g_critical("Failed to set device option '%s': %s.",
                   "samplerate", sr_strerror(ret));
        return ret;
    }

	return SR_OK;
}

void run_session(void)
{
	struct df_arg_desc df_arg;
    struct sr_dev_inst *mySaleaeLogic;

    mySaleaeLogic = NULL;
	GMainLoop *main_loop;

	memset(&df_arg, 0, sizeof(df_arg));
	df_arg.do_props = FALSE;

	GSList *devices = device_scan();
	if (!devices) {
		g_critical("No devices found.");
		return;
	}

	struct sr_dev_driver *driver;
	for (GSList *sd = devices; sd; sd = sd->next) {
        struct sr_dev_inst *sdi;
		sdi = sd->data;
		driver = sr_dev_inst_driver_get(sdi);

        GArray *drv_opts;
		if (!(drv_opts = sr_dev_options(driver, NULL, NULL))) {
			g_critical("Failed to query list of driver options.");
			return;
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
        return;
    }


    struct sr_session *session;
	sr_session_new(sr_ctx, &session);
	df_arg.session = session;
	sr_session_datafeed_callback_add(session, datafeed_in, &df_arg);
	df_arg.session = NULL;

	if (sr_dev_open(mySaleaeLogic) != SR_OK) {
		g_critical("Failed to open device.");
		return;
	}

	if (sr_session_dev_add(session, mySaleaeLogic) != SR_OK) {
		g_critical("Failed to add device to session.");
		sr_session_destroy(session);
		return;
	}

    if (set_dev_options(mySaleaeLogic) != SR_OK)
        return;


	if (select_channels(mySaleaeLogic) != SR_OK) {
		g_critical("Failed to set channels.");
		sr_session_destroy(session);
		return;
	}

    GVariant *gvar;
    if (maybe_config_list(driver, mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES,
            &gvar) == SR_OK) {
        /*
         * The device has no compression, or compression is turned
         * off, and publishes its sample memory size.
         */

        uint64_t min_samples, max_samples;
        g_variant_get(gvar, "(tt)", &min_samples, &max_samples);
        g_variant_unref(gvar);
        if (limit_samples < min_samples) {
            g_critical("The device stores at least %"PRIu64
                    " samples with the current settings.", min_samples);
        }
        if (limit_samples > max_samples) {
            g_critical("The device can store only %"PRIu64
                    " samples with the current settings.", max_samples);
        }
    }
    gvar = g_variant_new_uint64(limit_samples);
    if (maybe_config_set(sr_dev_inst_driver_get(mySaleaeLogic), mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES, gvar) != SR_OK) {
        g_critical("Failed to configure sample limit.");
        sr_session_destroy(session);
        return;
    }

	main_loop = g_main_loop_new(NULL, FALSE);

	sr_session_stopped_callback_set(session,
		(sr_session_stopped_callback)g_main_loop_quit, main_loop);

	if (sr_session_start(session) != SR_OK) {
		g_critical("Failed to start session.");
		g_main_loop_unref(main_loop);
		sr_session_destroy(session);
		return;
	}
	g_main_loop_run(main_loop);

	sr_session_datafeed_callback_remove_all(session);
	g_main_loop_unref(main_loop);
	sr_session_destroy(session);
}
