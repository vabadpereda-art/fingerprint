#ifndef ZKFP_SYNC_H
#define ZKFP_SYNC_H

#include "zkfp_common.h" // IWYU pragma: export

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_sync_config_reset(void);
int zkfp_sync_set_postgres_url(const char* postgres_url);
int zkfp_sync_set_interval_seconds(unsigned int seconds);
int zkfp_sync_set_daily_time(unsigned char hour, unsigned char minute);
int zkfp_sync_set_weekly_time(const char* weekdays_csv, unsigned char hour, unsigned char minute);
int zkfp_sync_set_cron(const char* cron_expression);
int zkfp_sync_set_manual(void);
int zkfp_sync_add_mapping(
    const char* postgres_query,
    const char* local_table,
    const char* mappings_json,
    int strategy,
    const char* unique_keys_csv
);
int zkfp_sync_apply_config(void);
int zkfp_sync_start(void);
int zkfp_sync_stop(void);
int zkfp_sync_run_now(void);
int zkfp_sync_is_running(void);
char* zkfp_sync_get_last_sync_at(void);

/* strategy values:
 * 0=Replace, 1=Append, 2=Upsert
 * weekdays_csv example: "mon,wed,fri"
 * mappings_json example: {"id":"user_id","name":"full_name"}
 */

#ifdef __cplusplus
}
#endif

#endif
