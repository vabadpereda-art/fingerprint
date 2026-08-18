//! Local SQLite storage with generic CRUD operations

use crate::error::{DbError, DbResult};
use crate::schema::{Schema, TableSchema};
use crate::types::{Row, Value};
use rusqlite::{Connection, Row as SqliteRow};
use std::sync::{Arc, Mutex};

/// Local SQLite database
pub struct LocalDb {
    conn: Arc<Mutex<Connection>>,
    schema: Arc<Mutex<Schema>>,
}

impl LocalDb {
    /// Open database from file path
    pub fn open(path: &str) -> DbResult<Self> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            schema: Arc::new(Mutex::new(Schema::new())),
        };
        db.init_metadata()?;
        db.load_schema()?;
        Ok(db)
    }

    /// Open in-memory database
    pub fn in_memory() -> DbResult<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
            schema: Arc::new(Mutex::new(Schema::new())),
        };
        db.init_metadata()?;
        Ok(db)
    }

    /// Initialize internal metadata tables
    fn init_metadata(&self) -> DbResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS _schema_registry (
                table_name TEXT PRIMARY KEY,
                schema_json TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS _sync_queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_name TEXT NOT NULL,
                row_id INTEGER NOT NULL,
                operation TEXT NOT NULL,
                data_json TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                synced INTEGER DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_sync_queue_pending
            ON _sync_queue(synced, created_at);
            ",
        )?;
        Ok(())
    }

    /// Load schema from metadata
    fn load_schema(&self) -> DbResult<()> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT table_name, schema_json FROM _schema_registry")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut schema = self.schema.lock().unwrap();
        for row in rows {
            let (_table_name, schema_json) = row?;
            let table_schema: TableSchema = serde_json::from_str(&schema_json)?;
            schema.add_table(table_schema).ok(); // Ignore duplicates
        }

        Ok(())
    }

    /// Create a new table
    pub fn create_table(&mut self, table_schema: TableSchema) -> DbResult<()> {
        let conn = self.conn.lock().unwrap();

        // Create the table
        let create_sql = table_schema.to_sqlite_create();
        conn.execute(&create_sql, [])?;

        // Store schema in registry
        let schema_json = serde_json::to_string(&table_schema)?;
        conn.execute(
            "INSERT OR REPLACE INTO _schema_registry (table_name, schema_json) VALUES (?1, ?2)",
            rusqlite::params![&table_schema.name, &schema_json],
        )?;

        // Add to in-memory schema
        let mut schema = self.schema.lock().unwrap();
        schema.add_table(table_schema)?;

        Ok(())
    }

    /// Get table schema
    pub fn get_table_schema(&self, table: &str) -> DbResult<TableSchema> {
        let schema = self.schema.lock().unwrap();
        schema
            .get_table(table)
            .cloned()
            .ok_or_else(|| DbError::TableNotFound(table.to_string()))
    }

    /// Insert a row
    pub fn insert(&self, table: &str, data: serde_json::Value) -> DbResult<i64> {
        let table_schema = self.get_table_schema(table)?;
        let conn = self.conn.lock().unwrap();

        // Extract columns and values from JSON
        let obj = data
            .as_object()
            .ok_or_else(|| DbError::Schema("Insert data must be a JSON object".to_string()))?;

        let mut columns = Vec::new();
        let mut values: Vec<Value> = Vec::new();

        for (key, val) in obj {
            // Skip internal columns
            if key.starts_with('_') || key == "id" {
                continue;
            }

            // Validate column exists
            if table_schema.get_column(key).is_none() {
                return Err(DbError::ColumnNotFound(key.clone()));
            }

            columns.push(key.clone());
            values.push(val.clone().into());
        }

        // Build INSERT statement
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("?{}", i)).collect();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );

        // Convert values to rusqlite params
        let params: Vec<&dyn rusqlite::ToSql> =
            values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();

        conn.execute(&sql, params.as_slice())?;
        let row_id = conn.last_insert_rowid();

        // Queue for sync
        self.queue_sync_operation(&conn, table, row_id, "INSERT", Some(&data))?;

        Ok(row_id)
    }

    /// Update a row by ID
    pub fn update(&self, table: &str, id: i64, data: serde_json::Value) -> DbResult<()> {
        let table_schema = self.get_table_schema(table)?;
        let conn = self.conn.lock().unwrap();

        let obj = data
            .as_object()
            .ok_or_else(|| DbError::Schema("Update data must be a JSON object".to_string()))?;

        let mut set_clauses = Vec::new();
        let mut values: Vec<Value> = Vec::new();

        for (key, val) in obj {
            if key.starts_with('_') || key == "id" {
                continue;
            }

            if table_schema.get_column(key).is_none() {
                return Err(DbError::ColumnNotFound(key.clone()));
            }

            set_clauses.push(format!("{} = ?", key));
            values.push(val.clone().into());
        }

        // Add updated_at timestamp
        set_clauses.push("_updated_at = CURRENT_TIMESTAMP".to_string());
        set_clauses.push("_synced = 0".to_string());

        let sql = format!(
            "UPDATE {} SET {} WHERE id = ?",
            table,
            set_clauses.join(", ")
        );

        let mut params: Vec<&dyn rusqlite::ToSql> =
            values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        params.push(&id);

        let affected = conn.execute(&sql, params.as_slice())?;
        if affected == 0 {
            return Err(DbError::Schema(format!(
                "Row {} not found in {}",
                id, table
            )));
        }

        // Queue for sync
        self.queue_sync_operation(&conn, table, id, "UPDATE", Some(&data))?;

        Ok(())
    }

    /// Delete a row by ID
    pub fn delete(&self, table: &str, id: i64) -> DbResult<()> {
        self.get_table_schema(table)?; // Validate table exists
        let conn = self.conn.lock().unwrap();

        let sql = format!("DELETE FROM {} WHERE id = ?", table);
        let affected = conn.execute(&sql, [id])?;

        if affected == 0 {
            return Err(DbError::Schema(format!(
                "Row {} not found in {}",
                id, table
            )));
        }

        // Queue for sync
        self.queue_sync_operation(&conn, table, id, "DELETE", None)?;

        Ok(())
    }

    /// Query rows (used by QueryBuilder)
    pub(crate) fn query_raw(&self, sql: &str, params: Vec<Value>) -> DbResult<Vec<Row>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|v| v as &dyn rusqlite::ToSql).collect();

        let column_count = stmt.column_count();
        let column_names: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap().to_string())
            .collect();

        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            Ok(Self::row_to_map(row, &column_names))
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }

    /// Convert SQLite row to our Row type
    fn row_to_map(row: &SqliteRow, column_names: &[String]) -> Row {
        let mut map = Row::new();
        for (i, name) in column_names.iter().enumerate() {
            let value = Self::extract_value(row, i);
            map.insert(name.clone(), value);
        }
        map
    }

    /// Extract value from SQLite row
    fn extract_value(row: &SqliteRow, idx: usize) -> Value {
        // Try different types in order
        if let Ok(v) = row.get::<_, i64>(idx) {
            return Value::Integer(v);
        }
        if let Ok(v) = row.get::<_, f64>(idx) {
            return Value::Real(v);
        }
        if let Ok(v) = row.get::<_, String>(idx) {
            return Value::Text(v);
        }
        if let Ok(v) = row.get::<_, Vec<u8>>(idx) {
            return Value::Blob(v);
        }
        Value::Null
    }

    /// Queue an operation for sync
    fn queue_sync_operation(
        &self,
        conn: &Connection,
        table: &str,
        row_id: i64,
        operation: &str,
        data: Option<&serde_json::Value>,
    ) -> DbResult<()> {
        let data_json = data.map(|d| d.to_string());
        conn.execute(
            "INSERT INTO _sync_queue (table_name, row_id, operation, data_json) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![table, row_id, operation, data_json],
        )?;
        Ok(())
    }

    /// Get pending sync operations
    pub fn get_pending_sync_operations(&self) -> DbResult<Vec<SyncOperation>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, table_name, row_id, operation, data_json, created_at
             FROM _sync_queue WHERE synced = 0 ORDER BY created_at",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(SyncOperation {
                id: row.get(0)?,
                table_name: row.get(1)?,
                row_id: row.get(2)?,
                operation: row.get(3)?,
                data_json: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;

        let mut operations = Vec::new();
        for row in rows {
            operations.push(row?);
        }

        Ok(operations)
    }

    /// Mark sync operation as completed
    pub fn mark_sync_completed(&self, sync_id: i64) -> DbResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE _sync_queue SET synced = 1 WHERE id = ?", [sync_id])?;
        Ok(())
    }

    /// Get connection (for advanced usage)
    pub fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }
}

/// Sync operation record
#[derive(Debug, Clone)]
pub struct SyncOperation {
    pub id: i64,
    pub table_name: String,
    pub row_id: i64,
    pub operation: String,
    pub data_json: Option<String>,
    pub created_at: String,
}

// Implement ToSql for Value
impl rusqlite::ToSql for Value {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, ValueRef};
        match self {
            Value::Null => Ok(ToSqlOutput::Borrowed(ValueRef::Null)),
            Value::Integer(i) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(*i))),
            Value::Real(f) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Real(*f))),
            Value::Text(s) => Ok(ToSqlOutput::Borrowed(ValueRef::Text(s.as_bytes()))),
            Value::Blob(b) => Ok(ToSqlOutput::Borrowed(ValueRef::Blob(b))),
            Value::Boolean(b) => Ok(ToSqlOutput::Owned(rusqlite::types::Value::Integer(if *b {
                1
            } else {
                0
            }))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{Column, ColumnConstraint, DataType};

    #[test]
    fn test_create_table() {
        let mut db = LocalDb::in_memory().unwrap();
        let mut schema = TableSchema::new("users".to_string());
        schema.add_column(Column::new(
            "name".to_string(),
            DataType::Text,
            vec![ColumnConstraint::NotNull],
        ));

        db.create_table(schema).unwrap();
        assert!(db.get_table_schema("users").is_ok());
    }

    #[test]
    fn test_insert_and_query() {
        let mut db = LocalDb::in_memory().unwrap();
        let mut schema = TableSchema::new("users".to_string());
        schema.add_column(Column::new(
            "name".to_string(),
            DataType::Text,
            vec![ColumnConstraint::NotNull],
        ));

        db.create_table(schema).unwrap();

        let id = db
            .insert("users", serde_json::json!({"name": "Alice"}))
            .unwrap();
        assert!(id > 0);
    }
}
