//! Dynamic schema definition and registry

use crate::error::{DbError, DbResult};
use crate::local::LocalDb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SQL data types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Integer,
    Real,
    Text,
    Blob,
    Boolean,
    Timestamp,
}

impl DataType {
    pub fn to_sqlite(&self) -> &'static str {
        match self {
            DataType::Integer => "INTEGER",
            DataType::Real => "REAL",
            DataType::Text => "TEXT",
            DataType::Blob => "BLOB",
            DataType::Boolean => "INTEGER", // SQLite stores booleans as integers
            DataType::Timestamp => "DATETIME",
        }
    }

    #[cfg(feature = "postgres-sync")]
    pub fn to_postgres(&self) -> &'static str {
        match self {
            DataType::Integer => "BIGINT",
            DataType::Real => "DOUBLE PRECISION",
            DataType::Text => "TEXT",
            DataType::Blob => "BYTEA",
            DataType::Boolean => "BOOLEAN",
            DataType::Timestamp => "TIMESTAMP",
        }
    }
}

/// Column constraints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColumnConstraint {
    PrimaryKey,
    NotNull,
    Unique,
    AutoIncrement,
    Default(String),
    ForeignKey { table: String, column: String },
}

/// Column definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<ColumnConstraint>,
}

impl Column {
    pub fn new(name: String, data_type: DataType, constraints: Vec<ColumnConstraint>) -> Self {
        Self {
            name,
            data_type,
            constraints,
        }
    }

    pub fn to_sqlite_ddl(&self) -> String {
        let mut parts = vec![self.name.clone(), self.data_type.to_sqlite().to_string()];

        for constraint in &self.constraints {
            match constraint {
                ColumnConstraint::PrimaryKey => parts.push("PRIMARY KEY".to_string()),
                ColumnConstraint::NotNull => parts.push("NOT NULL".to_string()),
                ColumnConstraint::Unique => parts.push("UNIQUE".to_string()),
                ColumnConstraint::AutoIncrement => parts.push("AUTOINCREMENT".to_string()),
                ColumnConstraint::Default(val) => parts.push(format!("DEFAULT {}", val)),
                ColumnConstraint::ForeignKey { table, column } => {
                    parts.push(format!("REFERENCES {}({})", table, column));
                }
            }
        }

        parts.join(" ")
    }

    #[cfg(feature = "postgres-sync")]
    pub fn to_postgres_ddl(&self) -> String {
        let mut parts = vec![self.name.clone(), self.data_type.to_postgres().to_string()];

        for constraint in &self.constraints {
            match constraint {
                ColumnConstraint::PrimaryKey => parts.push("PRIMARY KEY".to_string()),
                ColumnConstraint::NotNull => parts.push("NOT NULL".to_string()),
                ColumnConstraint::Unique => parts.push("UNIQUE".to_string()),
                ColumnConstraint::AutoIncrement => {
                    // PostgreSQL uses SERIAL or GENERATED ALWAYS AS IDENTITY
                    // We'll handle this in table creation
                }
                ColumnConstraint::Default(val) => parts.push(format!("DEFAULT {}", val)),
                ColumnConstraint::ForeignKey { table, column } => {
                    parts.push(format!("REFERENCES {}({})", table, column));
                }
            }
        }

        parts.join(" ")
    }
}

/// Table schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<Column>,
}

impl TableSchema {
    pub fn new(name: String) -> Self {
        Self {
            name,
            columns: Vec::new(),
        }
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|c| c.name == name)
    }

    pub fn to_sqlite_create(&self) -> String {
        let columns_ddl: Vec<String> = self.columns.iter().map(|c| c.to_sqlite_ddl()).collect();
        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n  {}\n)",
            self.name,
            columns_ddl.join(",\n  ")
        )
    }

    #[cfg(feature = "postgres-sync")]
    pub fn to_postgres_create(&self) -> String {
        let columns_ddl: Vec<String> = self.columns.iter().map(|c| c.to_postgres_ddl()).collect();
        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n  {}\n)",
            self.name,
            columns_ddl.join(",\n  ")
        )
    }
}

/// Schema registry - stores all table schemas
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schema {
    tables: HashMap<String, TableSchema>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, table: TableSchema) -> DbResult<()> {
        if self.tables.contains_key(&table.name) {
            return Err(DbError::Schema(format!(
                "Table '{}' already exists",
                table.name
            )));
        }
        self.tables.insert(table.name.clone(), table);
        Ok(())
    }

    pub fn get_table(&self, name: &str) -> Option<&TableSchema> {
        self.tables.get(name)
    }

    pub fn table_exists(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }

    pub fn list_tables(&self) -> Vec<&str> {
        self.tables.keys().map(|s| s.as_str()).collect()
    }
}

/// Builder for creating tables
pub struct TableBuilder<'a> {
    name: String,
    columns: Vec<Column>,
    db: &'a mut LocalDb,
}

impl<'a> TableBuilder<'a> {
    pub fn new(name: &str, db: &'a mut LocalDb) -> Self {
        Self {
            name: name.to_string(),
            columns: Vec::new(),
            db,
        }
    }

    pub fn column(
        mut self,
        name: &str,
        data_type: DataType,
        constraints: Vec<ColumnConstraint>,
    ) -> Self {
        self.columns.push(Column::new(
            name.to_string(),
            data_type,
            constraints,
        ));
        self
    }

    pub fn create(self) -> DbResult<()> {
        let mut schema = TableSchema::new(self.name.clone());
        
        // Always add internal tracking columns
        schema.add_column(Column::new(
            "id".to_string(),
            DataType::Integer,
            vec![ColumnConstraint::PrimaryKey, ColumnConstraint::AutoIncrement],
        ));

        // Add user-defined columns
        for col in self.columns {
            schema.add_column(col);
        }

        // Add sync tracking columns
        schema.add_column(Column::new(
            "_created_at".to_string(),
            DataType::Timestamp,
            vec![ColumnConstraint::Default("CURRENT_TIMESTAMP".to_string())],
        ));
        schema.add_column(Column::new(
            "_updated_at".to_string(),
            DataType::Timestamp,
            vec![ColumnConstraint::Default("CURRENT_TIMESTAMP".to_string())],
        ));
        schema.add_column(Column::new(
            "_synced".to_string(),
            DataType::Boolean,
            vec![ColumnConstraint::Default("0".to_string())],
        ));

        self.db.create_table(schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_ddl() {
        let col = Column::new(
            "name".to_string(),
            DataType::Text,
            vec![ColumnConstraint::NotNull],
        );
        assert_eq!(col.to_sqlite_ddl(), "name TEXT NOT NULL");
    }

    #[test]
    fn test_table_schema() {
        let mut schema = TableSchema::new("users".to_string());
        schema.add_column(Column::new(
            "id".to_string(),
            DataType::Integer,
            vec![ColumnConstraint::PrimaryKey],
        ));
        schema.add_column(Column::new(
            "name".to_string(),
            DataType::Text,
            vec![ColumnConstraint::NotNull],
        ));

        let ddl = schema.to_sqlite_create();
        assert!(ddl.contains("CREATE TABLE"));
        assert!(ddl.contains("users"));
    }

    #[test]
    fn test_schema_registry() {
        let mut registry = Schema::new();
        let schema = TableSchema::new("users".to_string());
        
        registry.add_table(schema).unwrap();
        assert!(registry.table_exists("users"));
        assert_eq!(registry.list_tables(), vec!["users"]);
    }
}
