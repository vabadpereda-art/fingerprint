//! # zkfp-db
//!
//! Generic ORM with offline-first SQLite storage and PostgreSQL synchronization.
//!
//! ## Features
//! - Dynamic schema definition (client defines tables/columns at runtime)
//! - Generic CRUD operations
//! - Offline-first with automatic sync to PostgreSQL
//! - Conflict resolution strategies
//! - Change tracking and delta sync
//!
//! ## Architecture
//! - `schema`: Dynamic schema registry and type system
//! - `local`: SQLite local storage with CRUD operations
//! - `query`: Generic query builder
//! - `sync`: Synchronization engine with PostgreSQL

pub mod error;
pub mod local;
pub mod query;
pub mod schema;
pub mod types;

#[cfg(feature = "postgres-sync")]
pub mod sync;

// Re-exports
pub use error::{DbError, DbResult};
pub use local::LocalDb;
pub use query::{QueryBuilder, WhereClause};
pub use schema::{ColumnConstraint, DataType, Schema, TableBuilder};
pub use types::{Row, Value};

#[cfg(feature = "postgres-sync")]
pub use sync::{SyncConfig, SyncEngine, SyncMapping, SyncSchedule, SyncStrategy};

/// Main database interface combining local storage and sync
pub struct ZkfpDb {
    local: LocalDb,
    #[cfg(feature = "postgres-sync")]
    sync_engine: Option<sync::SyncEngine>,
}

impl ZkfpDb {
    /// Create a new database instance
    pub fn new(local_path: &str) -> DbResult<Self> {
        let local = LocalDb::open(local_path)?;
        Ok(Self {
            local,
            #[cfg(feature = "postgres-sync")]
            sync_engine: None,
        })
    }

    /// Create an in-memory database
    pub fn in_memory() -> DbResult<Self> {
        let local = LocalDb::in_memory()?;
        Ok(Self {
            local,
            #[cfg(feature = "postgres-sync")]
            sync_engine: None,
        })
    }

    /// Configure PostgreSQL synchronization
    #[cfg(feature = "postgres-sync")]
    pub fn with_sync(mut self, config: SyncConfig) -> Self {
        self.sync_engine = Some(sync::SyncEngine::new(config));
        self
    }

    /// Start automatic synchronization daemon
    #[cfg(feature = "postgres-sync")]
    pub async fn start_sync(&mut self) -> DbResult<()> {
        if let Some(engine) = &mut self.sync_engine {
            engine.start(&self.local).await?;
        }
        Ok(())
    }

    /// Stop synchronization daemon
    #[cfg(feature = "postgres-sync")]
    pub async fn stop_sync(&mut self) -> DbResult<()> {
        if let Some(engine) = &mut self.sync_engine {
            engine.stop().await?;
        }
        Ok(())
    }

    /// Get reference to local database
    pub fn local(&self) -> &LocalDb {
        &self.local
    }

    /// Get mutable reference to local database
    pub fn local_mut(&mut self) -> &mut LocalDb {
        &mut self.local
    }

    /// Register a new table schema
    pub fn register_table(&mut self, name: &str) -> TableBuilder<'_> {
        TableBuilder::new(name, &mut self.local)
    }

    /// Insert a row into a table
    pub fn insert(&self, table: &str, data: serde_json::Value) -> DbResult<i64> {
        self.local.insert(table, data)
    }

    /// Query a table
    pub fn query(&self, table: &str) -> QueryBuilder<'_> {
        QueryBuilder::new(table, &self.local)
    }

    /// Update a row by ID
    pub fn update(&self, table: &str, id: i64, data: serde_json::Value) -> DbResult<()> {
        self.local.update(table, id, data)
    }

    /// Delete a row by ID
    pub fn delete(&self, table: &str, id: i64) -> DbResult<()> {
        self.local.delete(table, id)
    }

    /// Manually trigger a sync operation
    #[cfg(feature = "postgres-sync")]
    pub async fn sync_now(&self) -> DbResult<()> {
        if let Some(engine) = &self.sync_engine {
            engine.sync_once(&self.local).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_db() {
        let db = ZkfpDb::in_memory();
        assert!(db.is_ok());
    }

    #[test]
    fn test_register_and_insert() {
        let mut db = ZkfpDb::in_memory().unwrap();

        db.register_table("users")
            .column("name", DataType::Text, vec![ColumnConstraint::NotNull])
            .column("age", DataType::Integer, vec![])
            .create()
            .unwrap();

        let id = db
            .insert(
                "users",
                serde_json::json!({
                    "name": "Alice",
                    "age": 30
                }),
            )
            .unwrap();

        assert!(id > 0);
    }
}
