//! Generic query builder for dynamic queries

use crate::error::DbResult;
use crate::local::LocalDb;
use crate::types::{Row, Value};

/// Query builder for constructing SELECT queries
pub struct QueryBuilder<'a> {
    table: String,
    db: &'a LocalDb,
    where_clauses: Vec<WhereClause>,
    order_by: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(table: &str, db: &'a LocalDb) -> Self {
        Self {
            table: table.to_string(),
            db,
            where_clauses: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    /// Add a WHERE clause (column = value)
    pub fn where_eq(mut self, column: &str, value: impl Into<Value>) -> Self {
        self.where_clauses.push(WhereClause::Equals {
            column: column.to_string(),
            value: value.into(),
        });
        self
    }

    /// Add a WHERE clause (column != value)
    pub fn where_ne(mut self, column: &str, value: impl Into<Value>) -> Self {
        self.where_clauses.push(WhereClause::NotEquals {
            column: column.to_string(),
            value: value.into(),
        });
        self
    }

    /// Add a WHERE clause (column > value)
    pub fn where_gt(mut self, column: &str, value: impl Into<Value>) -> Self {
        self.where_clauses.push(WhereClause::GreaterThan {
            column: column.to_string(),
            value: value.into(),
        });
        self
    }

    /// Add a WHERE clause (column < value)
    pub fn where_lt(mut self, column: &str, value: impl Into<Value>) -> Self {
        self.where_clauses.push(WhereClause::LessThan {
            column: column.to_string(),
            value: value.into(),
        });
        self
    }

    /// Add a WHERE clause (column LIKE pattern)
    pub fn where_like(mut self, column: &str, pattern: &str) -> Self {
        self.where_clauses.push(WhereClause::Like {
            column: column.to_string(),
            pattern: pattern.to_string(),
        });
        self
    }

    /// Add a WHERE clause (column IS NULL)
    pub fn where_null(mut self, column: &str) -> Self {
        self.where_clauses.push(WhereClause::IsNull {
            column: column.to_string(),
        });
        self
    }

    /// Add a WHERE clause (column IS NOT NULL)
    pub fn where_not_null(mut self, column: &str) -> Self {
        self.where_clauses.push(WhereClause::IsNotNull {
            column: column.to_string(),
        });
        self
    }

    /// Add ORDER BY clause
    pub fn order_by(mut self, column: &str, ascending: bool) -> Self {
        let direction = if ascending { "ASC" } else { "DESC" };
        self.order_by = Some(format!("{} {}", column, direction));
        self
    }

    /// Add LIMIT clause
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Add OFFSET clause
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Execute query and fetch all results
    pub fn fetch_all(self) -> DbResult<Vec<Row>> {
        let (sql, params) = self.build_sql();
        self.db.query_raw(&sql, params)
    }

    /// Execute query and fetch first result
    pub fn fetch_one(self) -> DbResult<Option<Row>> {
        let (sql, params) = self.build_sql();
        let mut results = self.db.query_raw(&sql, params)?;
        Ok(results.pop())
    }

    /// Count matching rows
    pub fn count(self) -> DbResult<i64> {
        let (sql, params) = self.build_count_sql();
        let results = self.db.query_raw(&sql, params)?;
        if let Some(row) = results.first() {
            if let Some(count) = row.get("count") {
                return count.as_i64();
            }
        }
        Ok(0)
    }

    /// Build SQL query
    fn build_sql(&self) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT * FROM {}", self.table);
        let mut params = Vec::new();

        if !self.where_clauses.is_empty() {
            let (where_sql, where_params) = self.build_where_clause();
            sql.push_str(&format!(" WHERE {}", where_sql));
            params.extend(where_params);
        }

        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        (sql, params)
    }

    /// Build COUNT query
    fn build_count_sql(&self) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT COUNT(*) as count FROM {}", self.table);
        let mut params = Vec::new();

        if !self.where_clauses.is_empty() {
            let (where_sql, where_params) = self.build_where_clause();
            sql.push_str(&format!(" WHERE {}", where_sql));
            params.extend(where_params);
        }

        (sql, params)
    }

    /// Build WHERE clause
    fn build_where_clause(&self) -> (String, Vec<Value>) {
        let mut clauses = Vec::new();
        let mut params = Vec::new();

        for clause in &self.where_clauses {
            match clause {
                WhereClause::Equals { column, value } => {
                    clauses.push(format!("{} = ?", column));
                    params.push(value.clone());
                }
                WhereClause::NotEquals { column, value } => {
                    clauses.push(format!("{} != ?", column));
                    params.push(value.clone());
                }
                WhereClause::GreaterThan { column, value } => {
                    clauses.push(format!("{} > ?", column));
                    params.push(value.clone());
                }
                WhereClause::LessThan { column, value } => {
                    clauses.push(format!("{} < ?", column));
                    params.push(value.clone());
                }
                WhereClause::Like { column, pattern } => {
                    clauses.push(format!("{} LIKE ?", column));
                    params.push(Value::Text(pattern.clone()));
                }
                WhereClause::IsNull { column } => {
                    clauses.push(format!("{} IS NULL", column));
                }
                WhereClause::IsNotNull { column } => {
                    clauses.push(format!("{} IS NOT NULL", column));
                }
            }
        }

        (clauses.join(" AND "), params)
    }
}

/// WHERE clause types
#[derive(Debug, Clone)]
pub enum WhereClause {
    Equals { column: String, value: Value },
    NotEquals { column: String, value: Value },
    GreaterThan { column: String, value: Value },
    LessThan { column: String, value: Value },
    Like { column: String, pattern: String },
    IsNull { column: String },
    IsNotNull { column: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::local::LocalDb;
    use crate::schema::{Column, ColumnConstraint, DataType, TableSchema};

    fn setup_test_db() -> LocalDb {
        let mut db = LocalDb::in_memory().unwrap();
        let mut schema = TableSchema::new("users".to_string());
        schema.add_column(Column::new(
            "name".to_string(),
            DataType::Text,
            vec![ColumnConstraint::NotNull],
        ));
        schema.add_column(Column::new("age".to_string(), DataType::Integer, vec![]));

        db.create_table(schema).unwrap();

        db.insert("users", serde_json::json!({"name": "Alice", "age": 30}))
            .unwrap();
        db.insert("users", serde_json::json!({"name": "Bob", "age": 25}))
            .unwrap();
        db.insert("users", serde_json::json!({"name": "Charlie", "age": 35}))
            .unwrap();

        db
    }

    #[test]
    fn test_query_all() {
        let db = setup_test_db();
        let results = QueryBuilder::new("users", &db).fetch_all().unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_query_where_eq() {
        let db = setup_test_db();
        let results = QueryBuilder::new("users", &db)
            .where_eq("name", "Alice")
            .fetch_all()
            .unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_query_where_gt() {
        let db = setup_test_db();
        let results = QueryBuilder::new("users", &db)
            .where_gt("age", 25)
            .fetch_all()
            .unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_limit() {
        let db = setup_test_db();
        let results = QueryBuilder::new("users", &db)
            .limit(2)
            .fetch_all()
            .unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_count() {
        let db = setup_test_db();
        let count = QueryBuilder::new("users", &db).count().unwrap();
        assert_eq!(count, 3);
    }
}
