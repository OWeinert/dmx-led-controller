/*
 * This library is based from the sigrok-cli project.
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

#ifndef LIB_LOGIC_ANALYZER_H
#define LIB_LOGIC_ANALYZER_H

#include <stdbool.h>
#include <libsigrok/libsigrok.h>
#include <libsigrokdecode/libsigrokdecode.h>

#define MAX_LENGTH_DMX 22754e-6 // max dmx packet length: 22754 µs
#define LIMIT_SAMPLES(SAMPLE_RATE) ((uint64_t) ((2.1 *MAX_LENGTH_DMX) * SAMPLE_RATE))
#define CHANNEL 0   // Ch1 has index 0
#define PROTOCOL_DECODER "dmx512"
#define LOGIC_ANALYZER_DEVICE "fx2lafw"

/* libLogicAnalyzer.c */
struct CallbackData {
    void* rustData;
    void (*onDecoderAnnotation) (void*, struct srd_proto_data*);
};
__attribute__((unused)) int runAnalyzer(struct CallbackData* callbackData, bool fromDevice, uint64_t sampleRate);

/* device.c */
int device_init(struct sr_dev_inst **mySaleaeLogic, struct sr_context *sr_ctx, uint64_t sampleRate);

/* decode.c */
int sigrok_decode_session_start(struct srd_session **srd_sess, struct CallbackData* callbackData, gint opt_loglevel, struct srd_decoder_inst **di, struct sr_dev_inst *sdi, uint64_t sampleRate);

/* session.c */
struct cb_data {
    struct srd_session *srd_session;
    struct sr_session *sr_session;
    uint64_t limitSamples;
};
void sr_session_callback(const struct sr_dev_inst *sdi, const struct sr_datafeed_packet *packet, void *cb_data);
void run_session(struct sr_dev_inst *sdi, struct sr_context *sr_ctx, struct srd_session *srd_session, uint64_t sampleRate);

/* input.c */
void load_input_file(struct sr_context *sr_ctx, struct srd_session *srd_session, uint64_t sampleRate);

#endif
