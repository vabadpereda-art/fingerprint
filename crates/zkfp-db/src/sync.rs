//! PostgreSQL synchronization engine
//!
//! **Unidirectional sync: PostgreSQL → Local ONLY**
//!
//! Client defines:
//! 1. Custom SQL queries to fetch data from PostgreSQL
//! 2. Where to store query results in local tables
//! 3. When synchronization should run
//!
//! **NO data is pushed from local to PostgreSQL**

use crate::error::{DbError, DbResult};
use crate::local::LocalDb;
use chrono::{DateTime, Datelike, Utc, Weekday};
use cron::Schedule;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;
use tokio_postgres::{Client, NoTls};

#[derive(Debug, Clone)]
pub struct SyncMapping {
    pub postgres_query: String,
    pub local_table: String,
    pub column_map: HashMap<String, String>,
    pub strategy: SyncStrategy,
    pub unique_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStrategy {
    Replace,
    Append,
    Upsert,
}

impl SyncMapping {
    pub fn new(postgres_query: &str, local_table: &str) -> Self {
        Self {
            postgres_query: postgres_query.to_string(),
            local_table: local_table.to_string(),
            column_map: HashMap::new(),
            strategy: SyncStrategy::Upsert,
            unique_keys: vec!["id".to_string()],
        }
    }

    pub fn map_column(mut self, query_col: &str, local_col: &str) -> Self {
        self.column_map
            .insert(query_col.to_string(), local_col.to_string());
        self
    }

    pub fn map_same(mut self, column: &str) -> Self {
        self.column_map
            .insert(column.to_string(), column.to_string());
        self
    }

    pub fn with_strategy(mut self, strategy: SyncStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_unique_keys(mut self, keys: Vec<String>) -> Self {
        self.unique_keys = keys;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncSchedule {
    Manual,
    Interval {
        seconds: u64,
    },
    DailyAt {
        hour: u8,
        minute: u8,
    },
    Weekly {
        weekdays: Vec<Weekday>,
        hour: u8,
        minute: u8,
    },
    Cron {
        expression: String,
    },
}

impl SyncSchedule {
    pub fn interval(seconds: u64) -> Self {
        Self::Interval { seconds }
    }

    pub fn daily_at(hour: u8, minute: u8) -> Self {
        Self::DailyAt { hour, minute }
    }

    pub fn weekly(weekdays: Vec<Weekday>, hour: u8, minute: u8) -> Self {
        Self::Weekly {
            weekdays,
            hour,
            minute,
        }
    }

    pub fn cron(expression: impl Into<String>) -> Self {
        Self::Cron {
            expression: expression.into(),
        }
    }

    fn validate(&self) -> DbResult<()> {
        match self {
            SyncSchedule::Manual => Ok(()),
            SyncSchedule::Interval { seconds } => {
                if *seconds == 0 {
                    Err(DbError::Sync(
                        "Interval must be greater than 0 seconds".to_string(),
                    ))
                } else {
                    Ok(())
                }
            }
            SyncSchedule::DailyAt { hour, minute } => validate_time(*hour, *minute),
            SyncSchedule::Weekly {
                weekdays,
                hour,
                minute,
            } => {
                if weekdays.is_empty() {
                    return Err(DbError::Sync(
                        "Weekly schedule requires at least one weekday".to_string(),
                    ));
                }
                validate_time(*hour, *minute)
            }
            SyncSchedule::Cron { expression } => Schedule::from_str(expression)
                .map(|_| ())
                .map_err(|e| DbError::Sync(format!("Invalid cron expression: {e}"))),
        }
    }

    fn next_duration(&self, now: DateTime<Utc>) -> DbResult<Option<Duration>> {
        self.validate()?;
        match self {
            SyncSchedule::Manual => Ok(None),
            SyncSchedule::Interval { seconds } => Ok(Some(Duration::from_secs(*seconds))),
            SyncSchedule::DailyAt { hour, minute } => {
                let next = next_daily_occurrence(now, *hour, *minute)?;
                Ok(Some(
                    (next - now)
                        .to_std()
                        .map_err(|e| DbError::Sync(e.to_string()))?,
                ))
            }
            SyncSchedule::Weekly {
                weekdays,
                hour,
                minute,
            } => {
                let next = next_weekly_occurrence(now, weekdays, *hour, *minute)?;
                Ok(Some(
                    (next - now)
                        .to_std()
                        .map_err(|e| DbError::Sync(e.to_string()))?,
                ))
            }
            SyncSchedule::Cron { expression } => {
                let schedule =
                    Schedule::from_str(expression).map_err(|e| DbError::Sync(e.to_string()))?;
                let next = schedule.upcoming(Utc).next().ok_or_else(|| {
                    DbError::Sync("Cron schedule has no future occurrences".to_string())
                })?;
                Ok(Some(
                    (next - now)
                        .to_std()
                        .map_err(|e| DbError::Sync(e.to_string()))?,
                ))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub postgres_url: String,
    pub schedule: SyncSchedule,
    pub mappings: Vec<SyncMapping>,
    pub max_retries: u32,
    pub retry_backoff_secs: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            postgres_url: String::new(),
            schedule: SyncSchedule::Interval { seconds: 30 },
            mappings: Vec::new(),
            max_retries: 3,
            retry_backoff_secs: 5,
        }
    }
}

impl SyncConfig {
    pub fn new(postgres_url: String) -> Self {
        Self {
            postgres_url,
            ..Default::default()
        }
    }

    pub fn manual(mut self) -> Self {
        self.schedule = SyncSchedule::Manual;
        self
    }

    pub fn with_interval(mut self, secs: u64) -> Self {
        self.schedule = SyncSchedule::interval(secs);
        self
    }

    pub fn with_daily_schedule(mut self, hour: u8, minute: u8) -> Self {
        self.schedule = SyncSchedule::daily_at(hour, minute);
        self
    }

    pub fn with_weekly_schedule(mut self, weekdays: Vec<Weekday>, hour: u8, minute: u8) -> Self {
        self.schedule = SyncSchedule::weekly(weekdays, hour, minute);
        self
    }

    pub fn with_cron_schedule(mut self, expression: impl Into<String>) -> Self {
        self.schedule = SyncSchedule::cron(expression);
        self
    }

    pub fn add_mapping(mut self, mapping: SyncMapping) -> Self {
        self.mappings.push(mapping);
        self
    }
}

pub struct SyncEngine {
    config: SyncConfig,
    client: Arc<RwLock<Option<Client>>>,
    running: Arc<RwLock<bool>>,
    last_sync_at: Arc<RwLock<Option<DateTime<Utc>>>>,
}

impl SyncEngine {
    pub fn new(config: SyncConfig) -> Self {
        Self {
            config,
            client: Arc::new(RwLock::new(None)),
            running: Arc::new(RwLock::new(false)),
            last_sync_at: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start(&mut self, local_db: &LocalDb) -> DbResult<()> {
        self.config.schedule.validate()?;

        if matches!(self.config.schedule, SyncSchedule::Manual) {
            info!("Sync schedule = manual, daemon not started");
            return Ok(());
        }

        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        self.connect().await?;

        let config = self.config.clone();
        let client = Arc::clone(&self.client);
        let running = Arc::clone(&self.running);
        let last_sync_at = Arc::clone(&self.last_sync_at);
        let local_conn = local_db.connection();

        tokio::spawn(async move {
            loop {
                if !*running.read().await {
                    break;
                }

                let wait_duration = match config.schedule.next_duration(Utc::now()) {
                    Ok(Some(duration)) => duration,
                    Ok(None) => break,
                    Err(e) => {
                        error!("Invalid sync schedule: {}", e);
                        break;
                    }
                };

                debug!("Next sync in {:?}", wait_duration);
                time::sleep(wait_duration).await;

                if !*running.read().await {
                    break;
                }

                if client.read().await.is_none() {
                    warn!("No PostgreSQL connection, reconnecting...");
                    if let Err(e) = reconnect_client(&client, &config.postgres_url).await {
                        error!("Reconnect failed: {}", e);
                        continue;
                    }
                }

                let local_db = LocalDb::open(":memory:");
                let _ = local_db;
                let temp_local = LocalDbHandleAdapter::new(Arc::clone(&local_conn));

                match sync_once_with_shared_client(&client, &config, &temp_local).await {
                    Ok(_) => {
                        *last_sync_at.write().await = Some(Utc::now());
                        info!("Scheduled sync completed");
                    }
                    Err(e) => error!("Scheduled sync failed: {}", e),
                }
            }

            info!("Sync daemon stopped");
        });

        Ok(())
    }

    pub async fn stop(&mut self) -> DbResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        info!("Sync daemon stopped");
        Ok(())
    }

    async fn connect(&self) -> DbResult<()> {
        info!("Connecting to PostgreSQL...");

        match tokio_postgres::connect(&self.config.postgres_url, NoTls).await {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        error!("PostgreSQL connection error: {}", e);
                    }
                });

                *self.client.write().await = Some(client);
                info!("Connected to PostgreSQL");
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect: {}", e);
                Err(DbError::Connection(e.to_string()))
            }
        }
    }

    pub async fn is_connected(&self) -> bool {
        self.client.read().await.is_some()
    }

    pub async fn last_sync_at(&self) -> Option<DateTime<Utc>> {
        *self.last_sync_at.read().await
    }

    pub async fn sync_once(&self, local_db: &LocalDb) -> DbResult<()> {
        if !self.is_connected().await {
            self.connect().await?;
        }

        info!("Starting sync (PostgreSQL → Local)...");

        let client = self.client.read().await;
        let client = client
            .as_ref()
            .ok_or_else(|| DbError::Connection("PostgreSQL client not connected".to_string()))?;

        for mapping in &self.config.mappings {
            match self.execute_sync(client, mapping, local_db).await {
                Ok(count) => info!("Synced {} rows → '{}'", count, mapping.local_table),
                Err(e) => error!("Sync failed for '{}': {}", mapping.local_table, e),
            }
        }

        *self.last_sync_at.write().await = Some(Utc::now());
        info!("Sync completed");
        Ok(())
    }

    async fn execute_sync(
        &self,
        client: &Client,
        mapping: &SyncMapping,
        local_db: &LocalDb,
    ) -> DbResult<usize> {
        execute_sync_impl(client, mapping, local_db).await
    }
}

async fn sync_once_with_shared_client(
    client_lock: &Arc<RwLock<Option<Client>>>,
    config: &SyncConfig,
    local_db: &LocalDbHandleAdapter,
) -> DbResult<()> {
    let client_guard = client_lock.read().await;
    let client = client_guard
        .as_ref()
        .ok_or_else(|| DbError::Connection("PostgreSQL client not connected".to_string()))?;

    for mapping in &config.mappings {
        match execute_sync_impl_with_adapter(client, mapping, local_db).await {
            Ok(count) => info!("Synced {} rows → '{}'", count, mapping.local_table),
            Err(e) => error!("Sync failed for '{}': {}", mapping.local_table, e),
        }
    }

    Ok(())
}

async fn reconnect_client(
    client_lock: &Arc<RwLock<Option<Client>>>,
    postgres_url: &str,
) -> DbResult<()> {
    match tokio_postgres::connect(postgres_url, NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    error!("PostgreSQL connection error: {}", e);
                }
            });
            *client_lock.write().await = Some(client);
            Ok(())
        }
        Err(e) => Err(DbError::Connection(e.to_string())),
    }
}

async fn execute_sync_impl(
    client: &Client,
    mapping: &SyncMapping,
    local_db: &LocalDb,
) -> DbResult<usize> {
    debug!("Executing query: {}", mapping.postgres_query);

    let rows = client.query(&mapping.postgres_query, &[]).await?;

    if mapping.strategy == SyncStrategy::Replace {
        let conn = local_db.connection();
        conn.lock()
            .unwrap()
            .execute(&format!("DELETE FROM {}", mapping.local_table), [])?;
    }

    let mut count = 0;
    for pg_row in rows {
        let mut local_data = serde_json::Map::new();

        for (query_col, local_col) in &mapping.column_map {
            if let Ok(Some(v)) = pg_row.try_get::<_, Option<String>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::String(v));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<i64>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::Number(v.into()));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<i32>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::Number(v.into()));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<f64>>(query_col.as_str()) {
                if let Some(num) = serde_json::Number::from_f64(v) {
                    local_data.insert(local_col.clone(), serde_json::Value::Number(num));
                }
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<bool>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::Bool(v));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<Vec<u8>>>(query_col.as_str()) {
                local_data.insert(
                    local_col.clone(),
                    serde_json::Value::Array(
                        v.into_iter()
                            .map(|b| serde_json::Value::Number(b.into()))
                            .collect(),
                    ),
                );
            }
        }

        match mapping.strategy {
            SyncStrategy::Replace | SyncStrategy::Append => {
                local_db.insert(&mapping.local_table, serde_json::Value::Object(local_data))?;
            }
            SyncStrategy::Upsert => {
                let conn = local_db.connection();
                let conn_lock = conn.lock().unwrap();
                let columns: Vec<String> = local_data.keys().cloned().collect();
                let placeholders: Vec<String> =
                    (1..=columns.len()).map(|i| format!("?{}", i)).collect();
                let sql = format!(
                    "REPLACE INTO {} ({}) VALUES ({})",
                    mapping.local_table,
                    columns.join(", "),
                    placeholders.join(", ")
                );
                let values: Vec<rusqlite::types::Value> =
                    local_data.values().map(json_to_sqlite_value).collect();
                conn_lock.execute(&sql, rusqlite::params_from_iter(values))?;
            }
        }

        count += 1;
    }

    Ok(count)
}

async fn execute_sync_impl_with_adapter(
    client: &Client,
    mapping: &SyncMapping,
    local_db: &LocalDbHandleAdapter,
) -> DbResult<usize> {
    debug!("Executing query: {}", mapping.postgres_query);

    let rows = client.query(&mapping.postgres_query, &[]).await?;

    if mapping.strategy == SyncStrategy::Replace {
        local_db.delete_all(&mapping.local_table)?;
    }

    let mut count = 0;
    for pg_row in rows {
        let mut local_data = serde_json::Map::new();

        for (query_col, local_col) in &mapping.column_map {
            if let Ok(Some(v)) = pg_row.try_get::<_, Option<String>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::String(v));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<i64>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::Number(v.into()));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<i32>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::Number(v.into()));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<f64>>(query_col.as_str()) {
                if let Some(num) = serde_json::Number::from_f64(v) {
                    local_data.insert(local_col.clone(), serde_json::Value::Number(num));
                }
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<bool>>(query_col.as_str()) {
                local_data.insert(local_col.clone(), serde_json::Value::Bool(v));
            } else if let Ok(Some(v)) = pg_row.try_get::<_, Option<Vec<u8>>>(query_col.as_str()) {
                local_data.insert(
                    local_col.clone(),
                    serde_json::Value::Array(
                        v.into_iter()
                            .map(|b| serde_json::Value::Number(b.into()))
                            .collect(),
                    ),
                );
            }
        }

        match mapping.strategy {
            SyncStrategy::Replace | SyncStrategy::Append => {
                local_db.insert(&mapping.local_table, serde_json::Value::Object(local_data))?;
            }
            SyncStrategy::Upsert => {
                local_db.upsert(&mapping.local_table, serde_json::Value::Object(local_data))?;
            }
        }

        count += 1;
    }

    Ok(count)
}

struct LocalDbHandleAdapter {
    conn: Arc<std::sync::Mutex<rusqlite::Connection>>,
}

impl LocalDbHandleAdapter {
    fn new(conn: Arc<std::sync::Mutex<rusqlite::Connection>>) -> Self {
        Self { conn }
    }

    fn insert(&self, table: &str, data: serde_json::Value) -> DbResult<i64> {
        let conn = self.conn.lock().unwrap();
        let obj = data
            .as_object()
            .ok_or_else(|| DbError::Schema("Insert data must be a JSON object".to_string()))?;

        let columns: Vec<String> = obj.keys().cloned().collect();
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("?{}", i)).collect();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );
        let values: Vec<rusqlite::types::Value> = obj.values().map(json_to_sqlite_value).collect();
        conn.execute(&sql, rusqlite::params_from_iter(values))?;
        Ok(conn.last_insert_rowid())
    }

    fn upsert(&self, table: &str, data: serde_json::Value) -> DbResult<i64> {
        let conn = self.conn.lock().unwrap();
        let obj = data
            .as_object()
            .ok_or_else(|| DbError::Schema("Upsert data must be a JSON object".to_string()))?;

        let columns: Vec<String> = obj.keys().cloned().collect();
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("?{}", i)).collect();
        let sql = format!(
            "REPLACE INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );
        let values: Vec<rusqlite::types::Value> = obj.values().map(json_to_sqlite_value).collect();
        conn.execute(&sql, rusqlite::params_from_iter(values))?;
        Ok(conn.last_insert_rowid())
    }

    fn delete_all(&self, table: &str) -> DbResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(&format!("DELETE FROM {}", table), [])?;
        Ok(())
    }
}

fn json_to_sqlite_value(value: &serde_json::Value) -> rusqlite::types::Value {
    match value {
        serde_json::Value::Null => rusqlite::types::Value::Null,
        serde_json::Value::Bool(b) => rusqlite::types::Value::Integer(if *b { 1 } else { 0 }),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                rusqlite::types::Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                rusqlite::types::Value::Real(f)
            } else {
                rusqlite::types::Value::Null
            }
        }
        serde_json::Value::String(s) => rusqlite::types::Value::Text(s.clone()),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            rusqlite::types::Value::Text(value.to_string())
        }
    }
}

fn validate_time(hour: u8, minute: u8) -> DbResult<()> {
    if hour > 23 {
        return Err(DbError::Sync("Hour must be between 0 and 23".to_string()));
    }
    if minute > 59 {
        return Err(DbError::Sync("Minute must be between 0 and 59".to_string()));
    }
    Ok(())
}

fn next_daily_occurrence(now: DateTime<Utc>, hour: u8, minute: u8) -> DbResult<DateTime<Utc>> {
    validate_time(hour, minute)?;
    let today = now.date_naive();
    let today_candidate = today
        .and_hms_opt(hour as u32, minute as u32, 0)
        .ok_or_else(|| DbError::Sync("Invalid daily schedule time".to_string()))?
        .and_utc();

    if today_candidate > now {
        Ok(today_candidate)
    } else {
        let tomorrow = today
            .succ_opt()
            .ok_or_else(|| DbError::Sync("Could not compute next daily schedule".to_string()))?;
        Ok(tomorrow
            .and_hms_opt(hour as u32, minute as u32, 0)
            .ok_or_else(|| DbError::Sync("Invalid daily schedule time".to_string()))?
            .and_utc())
    }
}

fn next_weekly_occurrence(
    now: DateTime<Utc>,
    weekdays: &[Weekday],
    hour: u8,
    minute: u8,
) -> DbResult<DateTime<Utc>> {
    validate_time(hour, minute)?;
    let mut best: Option<DateTime<Utc>> = None;

    for day in weekdays {
        let current_num = now.weekday().num_days_from_monday() as i64;
        let target_num = day.num_days_from_monday() as i64;
        let mut delta = target_num - current_num;
        if delta < 0 {
            delta += 7;
        }

        let date = now
            .date_naive()
            .checked_add_signed(chrono::Duration::days(delta))
            .ok_or_else(|| DbError::Sync("Could not compute weekly schedule date".to_string()))?;
        let candidate = date
            .and_hms_opt(hour as u32, minute as u32, 0)
            .ok_or_else(|| DbError::Sync("Invalid weekly schedule time".to_string()))?
            .and_utc();

        let candidate = if candidate <= now {
            let next_date = date
                .checked_add_signed(chrono::Duration::days(7))
                .ok_or_else(|| {
                    DbError::Sync("Could not compute next weekly schedule".to_string())
                })?;
            next_date
                .and_hms_opt(hour as u32, minute as u32, 0)
                .ok_or_else(|| DbError::Sync("Invalid weekly schedule time".to_string()))?
                .and_utc()
        } else {
            candidate
        };

        if best.map(|b| candidate < b).unwrap_or(true) {
            best = Some(candidate);
        }
    }

    best.ok_or_else(|| DbError::Sync("Weekly schedule produced no occurrences".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_mapping() {
        let mapping = SyncMapping::new(
            "SELECT id, name FROM users WHERE active = true",
            "local_users",
        )
        .map_same("id")
        .map_column("name", "full_name")
        .with_strategy(SyncStrategy::Upsert);

        assert_eq!(mapping.local_table, "local_users");
        assert_eq!(
            mapping.column_map.get("name"),
            Some(&"full_name".to_string())
        );
        assert_eq!(mapping.strategy, SyncStrategy::Upsert);
    }

    #[test]
    fn test_schedule_validation() {
        assert!(SyncSchedule::interval(30).validate().is_ok());
        assert!(SyncSchedule::daily_at(23, 59).validate().is_ok());
        assert!(SyncSchedule::daily_at(24, 0).validate().is_err());
        assert!(SyncSchedule::weekly(vec![Weekday::Mon], 12, 0)
            .validate()
            .is_ok());
        assert!(SyncSchedule::weekly(vec![], 12, 0).validate().is_err());
    }

    #[test]
    fn test_sync_config() {
        let mapping = SyncMapping::new("SELECT * FROM users", "local_users")
            .map_same("id")
            .map_same("name");

        let config = SyncConfig::new("postgres://localhost/db".to_string())
            .with_daily_schedule(3, 30)
            .add_mapping(mapping);

        assert_eq!(config.mappings.len(), 1);
        assert!(matches!(
            config.schedule,
            SyncSchedule::DailyAt {
                hour: 3,
                minute: 30
            }
        ));
    }
}
