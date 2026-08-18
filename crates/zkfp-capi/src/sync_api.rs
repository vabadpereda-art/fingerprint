use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint};

use chrono::Weekday;
use serde_json::Value as JsonValue;
use tokio::runtime::Runtime;
use zkfp_db::{SyncConfig, SyncEngine, SyncMapping, SyncSchedule, SyncStrategy};

use crate::common::{DATABASE, RUNTIME, SYNC_CONFIG, SYNC_ENGINE, cstr_to_str, set_error};

fn with_runtime<T>(f: impl FnOnce(&Runtime) -> Result<T, ()>) -> Result<T, ()> {
    f(&RUNTIME)
}

fn parse_strategy(strategy: c_int) -> Result<SyncStrategy, ()> {
    match strategy {
        0 => Ok(SyncStrategy::Replace),
        1 => Ok(SyncStrategy::Append),
        2 => Ok(SyncStrategy::Upsert),
        _ => {
            set_error("Invalid sync strategy");
            Err(())
        }
    }
}

fn parse_weekday(token: &str) -> Option<Weekday> {
    match token.trim().to_lowercase().as_str() {
        "mon" | "monday" => Some(Weekday::Mon),
        "tue" | "tues" | "tuesday" => Some(Weekday::Tue),
        "wed" | "wednesday" => Some(Weekday::Wed),
        "thu" | "thurs" | "thursday" => Some(Weekday::Thu),
        "fri" | "friday" => Some(Weekday::Fri),
        "sat" | "saturday" => Some(Weekday::Sat),
        "sun" | "sunday" => Some(Weekday::Sun),
        _ => None,
    }
}

fn parse_csv(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect()
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_config_reset() -> c_int {
    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            *cfg = SyncConfig::default();
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_set_postgres_url(postgres_url: *const c_char) -> c_int {
    let postgres_url = match cstr_to_str(postgres_url, "postgres_url") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            cfg.postgres_url = postgres_url.to_string();
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_set_interval_seconds(seconds: c_uint) -> c_int {
    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            cfg.schedule = SyncSchedule::interval(seconds as u64);
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_set_daily_time(hour: c_uchar, minute: c_uchar) -> c_int {
    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            cfg.schedule = SyncSchedule::daily_at(hour, minute);
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_set_weekly_time(
    weekdays_csv: *const c_char,
    hour: c_uchar,
    minute: c_uchar,
) -> c_int {
    let weekdays_csv = match cstr_to_str(weekdays_csv, "weekdays_csv") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    let weekdays: Vec<Weekday> = parse_csv(weekdays_csv)
        .into_iter()
        .filter_map(|s| parse_weekday(&s))
        .collect();

    if weekdays.is_empty() {
        set_error("No valid weekdays provided");
        return 0;
    }

    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            cfg.schedule = SyncSchedule::weekly(weekdays, hour, minute);
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_set_cron(cron_expression: *const c_char) -> c_int {
    let cron_expression = match cstr_to_str(cron_expression, "cron_expression") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            cfg.schedule = SyncSchedule::cron(cron_expression.to_string());
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_set_manual() -> c_int {
    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            cfg.schedule = SyncSchedule::Manual;
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_add_mapping(
    postgres_query: *const c_char,
    local_table: *const c_char,
    mappings_json: *const c_char,
    strategy: c_int,
    unique_keys_csv: *const c_char,
) -> c_int {
    let postgres_query = match cstr_to_str(postgres_query, "postgres_query") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let local_table = match cstr_to_str(local_table, "local_table") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let mappings_json = match cstr_to_str(mappings_json, "mappings_json") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let strategy = match parse_strategy(strategy) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let unique_keys = if unique_keys_csv.is_null() {
        vec!["id".to_string()]
    } else {
        match cstr_to_str(unique_keys_csv, "unique_keys_csv") {
            Ok(v) => parse_csv(v),
            Err(_) => return 0,
        }
    };

    let parsed: JsonValue = match serde_json::from_str(mappings_json) {
        Ok(v) => v,
        Err(e) => {
            set_error(&format!("Invalid mappings_json: {e}"));
            return 0;
        }
    };
    let obj = match parsed.as_object() {
        Some(v) => v,
        None => {
            set_error("mappings_json must be a JSON object");
            return 0;
        }
    };

    let mut column_map = HashMap::new();
    for (k, v) in obj {
        let target = match v.as_str() {
            Some(s) => s,
            None => {
                set_error("mappings_json values must be strings");
                return 0;
            }
        };
        column_map.insert(k.clone(), target.to_string());
    }

    match SYNC_CONFIG.lock() {
        Ok(mut cfg) => {
            cfg.mappings.push(SyncMapping {
                postgres_query: postgres_query.to_string(),
                local_table: local_table.to_string(),
                column_map,
                strategy,
                unique_keys,
            });
            1
        }
        Err(_) => {
            set_error("Sync config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_apply_config() -> c_int {
    let cfg = match SYNC_CONFIG.lock() {
        Ok(cfg) => cfg.clone(),
        Err(_) => {
            set_error("Sync config mutex poisoned");
            return 0;
        }
    };

    match SYNC_ENGINE.lock() {
        Ok(mut engine_slot) => {
            *engine_slot = Some(SyncEngine::new(cfg));
            1
        }
        Err(_) => {
            set_error("Sync engine mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_start() -> c_int {
    let mut engine_guard = match SYNC_ENGINE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Sync engine mutex poisoned");
            return 0;
        }
    };
    let engine = match engine_guard.as_mut() {
        Some(engine) => engine,
        None => {
            set_error("Sync engine not configured");
            return 0;
        }
    };

    let db_guard = match DATABASE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Database mutex poisoned");
            return 0;
        }
    };
    let db = match db_guard.as_ref() {
        Some(db) => db,
        None => {
            set_error("Database not initialized");
            return 0;
        }
    };

    match with_runtime(|rt| {
        rt.block_on(engine.start(db.local())).map_err(|e| {
            set_error(&format!("Failed to start sync: {e}"));
        })
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_stop() -> c_int {
    let mut engine_guard = match SYNC_ENGINE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Sync engine mutex poisoned");
            return 0;
        }
    };
    let engine = match engine_guard.as_mut() {
        Some(engine) => engine,
        None => {
            set_error("Sync engine not configured");
            return 0;
        }
    };

    match with_runtime(|rt| {
        rt.block_on(engine.stop()).map_err(|e| {
            set_error(&format!("Failed to stop sync: {e}"));
        })
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_run_now() -> c_int {
    let engine_guard = match SYNC_ENGINE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Sync engine mutex poisoned");
            return 0;
        }
    };
    let engine = match engine_guard.as_ref() {
        Some(engine) => engine,
        None => {
            set_error("Sync engine not configured");
            return 0;
        }
    };

    let db_guard = match DATABASE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Database mutex poisoned");
            return 0;
        }
    };
    let db = match db_guard.as_ref() {
        Some(db) => db,
        None => {
            set_error("Database not initialized");
            return 0;
        }
    };

    match with_runtime(|rt| {
        rt.block_on(engine.sync_once(db.local())).map_err(|e| {
            set_error(&format!("Failed to run sync: {e}"));
        })
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_is_running() -> c_int {
    let engine_guard = match SYNC_ENGINE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Sync engine mutex poisoned");
            return 0;
        }
    };
    if engine_guard.is_some() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_sync_get_last_sync_at() -> *mut c_char {
    let engine_guard = match SYNC_ENGINE.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Sync engine mutex poisoned");
            return std::ptr::null_mut();
        }
    };
    let engine = match engine_guard.as_ref() {
        Some(engine) => engine,
        None => {
            set_error("Sync engine not configured");
            return std::ptr::null_mut();
        }
    };

    match with_runtime(|rt| {
        let value = rt.block_on(engine.last_sync_at());
        let text = value.map(|dt| dt.to_rfc3339()).unwrap_or_default();
        Ok(match CString::new(text) {
            Ok(cs) => cs.into_raw(),
            Err(_) => {
                set_error("Invalid sync timestamp string");
                std::ptr::null_mut()
            }
        })
    }) {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}
