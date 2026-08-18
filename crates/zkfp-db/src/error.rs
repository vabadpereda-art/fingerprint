//! Error types for zkfp-db

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Template error: {0}")]
    Template(#[from] zkfp_template::TemplateError),

    #[cfg(feature = "postgres-sync")]
    #[error("PostgreSQL error: {0}")]
    Postgres(#[from] tokio_postgres::Error),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Table not found: {0}")]
    TableNotFound(String),

    #[error("Column not found: {0}")]
    ColumnNotFound(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Sync error: {0}")]
    Sync(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("User not found: {0}")]
    UserNotFound(u32),

    #[error("Duplicate user: {0}")]
    DuplicateUser(u32),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type DbResult<T> = Result<T, DbError>;
