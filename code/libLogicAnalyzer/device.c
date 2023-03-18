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

GSList *device_scan(struct sr_context *sr_ctx);
int set_dev_options(struct sr_dev_inst *sdi);

int device_init(struct sr_dev_inst **mySaleaeLogic, struct sr_context *sr_ctx) {
    GSList *devices = device_scan(sr_ctx);
    if (!devices) {
        g_critical("No devices found, try reconnecting the logic analyzer");
        return -1;
    }

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

        if(g_str_equal(driver->name, LOGIC_ANALYZER_DEVICE) ) {
            *mySaleaeLogic = sdi;
            break;
        }
    }
    g_slist_free(devices);

    if (*mySaleaeLogic == NULL) {
        g_critical("No real devices found.");
        return -1;
    }

    if (sr_dev_open(*mySaleaeLogic) != SR_OK) {
        g_critical("Failed to open device.");
        return -1;
    }

    if (set_dev_options(*mySaleaeLogic) != SR_OK) {
        g_critical("logic analyzer: Error 300 occurred, exiting");
        return -1;
    }

    if (!(sr_dev_config_capabilities_list(*mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES) & SR_CONF_SET)
        || sr_config_set(*mySaleaeLogic, NULL, SR_CONF_LIMIT_SAMPLES, g_variant_new_uint64(LIMIT_SAMPLES)) != SR_OK)
    {
        g_critical("Failed to configure sample limit.");
        return -1;
    }

    return SR_OK;
}

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