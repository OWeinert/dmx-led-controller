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
#include "logicAnalyzer.h"

GSList *device_scan(struct sr_context *sr_ctx)
{
	struct sr_dev_driver *driver = NULL;
    struct sr_dev_driver **drivers = sr_driver_list(sr_ctx);
    for (int i = 0; drivers[i]; i++) {
        if (strcmp(drivers[i]->name, "fx2lafw") != 0)
            continue;
        driver = drivers[i];
        break;
    }
    if (!driver) {
        g_critical("Driver %s not found.", "fx2lafw");
        return NULL;
    }
    if (sr_driver_init(sr_ctx, driver) != SR_OK) {
        g_critical("Failed to initialize driver.");
        return NULL;
    }

	GSList *devices = sr_driver_scan(driver, NULL);

	return devices;
}

int set_dev_options(struct sr_dev_inst *sdi)
{
    struct sr_config src;
    const struct sr_key_info *srci;

    srci = sr_key_info_name_get(SR_KEY_CONFIG, "samplerate");
    if (!srci) {
        g_critical("Unknown device option '%s'.", "samplerate");
        return SR_ERR;
    }
    src.key = srci->key;
    src.data = g_variant_new_uint64(SAMPLE_RATE);

    if (!(sr_dev_config_capabilities_list(sdi, NULL, src.key) & SR_CONF_SET)){
        g_critical("Unknown key: %s : %s.", "samplerate", sr_strerror(SR_ERR_NA));
        return SR_ERR_NA;
    }
    if (sr_config_set(sdi, NULL, src.key, src.data) != SR_OK) {
        g_critical("Failed to set device option '%s': %s.", "samplerate", sr_strerror(SR_ERR_NA));
        return SR_ERR_NA;
    }
    return SR_OK;
}