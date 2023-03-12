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

#include <libsigrok/libsigrok.h>
#include <libsigrokdecode/libsigrokdecode.h>

#define SAMPLE_RATE ((uint64_t) 24e6) // 24 MHz
#define MAX_LENGTH_DMX 22754e-6 // max dmx packet length: 22754 µs
#define LIMIT_SAMPLES ((uint64_t) ((2.1 *MAX_LENGTH_DMX) * SAMPLE_RATE))

/* main.c */
extern gint opt_loglevel;
struct CallbackData {
    void* rustData;
    void (*onDecoderAnnotation) (void*, struct srd_proto_data*);
};

__attribute__((unused)) int runAnalyzer(struct CallbackData* callbackData);

/* device.c */
GSList *device_scan(struct sr_context *sr_ctx);
int set_dev_options(struct sr_dev_inst *sdi);

/* session.c */
struct cb_data {
    struct srd_session *srd_session;
    struct sr_session *sr_session;
    int receivingEnabled;
    struct srd_decoder_inst *di;
};
void sr_session_callback(const struct sr_dev_inst *sdi,
                         const struct sr_datafeed_packet *packet, void *cb_data);
void run_session(struct sr_dev_inst *sdi, struct sr_context *sr_ctx, struct srd_session *srd_session, struct srd_decoder_inst *di);

/* decode.c */
void srd_callback(struct srd_proto_data *pdata, void *cb_data);
int  register_pd_with_channels(struct sr_dev_inst *sdi, struct srd_decoder_inst *di);

#endif
