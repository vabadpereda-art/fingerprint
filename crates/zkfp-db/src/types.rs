//! Generic type system for dynamic data

use crate::error::{DbError, DbResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generic value type that can represent any SQLite/PostgreSQL value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
    Boolean(bool),
}

impl Value {
    pub fn as_i64(&self) -> DbResult<i64> {
        match self {
            Value::Integer(i) => Ok(*i),
            _ => Err(DbError::TypeMismatch {
                expected: "Integer".to_string(),
                actual: format!("{:?}", self),
            }),
        }
    }

    pub fn as_f64(&self) -> DbResult<f64> {
        match self {
            Value::Real(f) => Ok(*f),
            Value::Integer(i) => Ok(*i as f64),
            _ => Err(DbError::TypeMismatch {
                expected: "Real".to_string(),
                actual: format!("{:?}", self),
            }),
        }
    }

    pub fn as_str(&self) -> DbResult<&str> {
        match self {
            Value::Text(s) => Ok(s),
            _ => Err(DbError::TypeMismatch {
                expected: "Text".to_string(),
                actual: format!("{:?}", self),
            }),
        }
    }

    pub fn as_bytes(&self) -> DbResult<&[u8]> {
        match self {
            Value::Blob(b) => Ok(b),
            _ => Err(DbError::TypeMismatch {
                expected: "Blob".to_string(),
                actual: format!("{:?}", self),
            }),
        }
    }

    pub fn as_bool(&self) -> DbResult<bool> {
        match self {
            Value::Boolean(b) => Ok(*b),
            Value::Integer(i) => Ok(*i != 0),
            _ => Err(DbError::TypeMismatch {
                expected: "Boolean".to_string(),
                actual: format!("{:?}", self),
            }),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v as i64)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Real(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(val) => val.into(),
            None => Value::Null,
        }
    }
}

/// A row of data (column name -> value mapping)
pub type Row = HashMap<String, Value>;

/// Convert serde_json::Value to our Value type
impl From<serde_json::Value> for Value {
    fn from(v: serde_json::Value) -> Self {
        match v {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Boolean(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Real(f)
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::Text(s),
            serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                // Serialize complex types as JSON strings
                Value::Text(v.to_string())
            }
        }
    }
}

/// Convert our Value type to serde_json::Value
impl From<Value> for serde_json::Value {
    fn from(v: Value) -> Self {
        match v {
            Value::Null => serde_json::Value::Null,
            Value::Integer(i) => serde_json::Value::Number(i.into()),
            Value::Real(f) => serde_json::Value::Number(
                serde_json::Number::from_f64(f).unwrap_or(0.into()),
            ),
            Value::Text(s) => serde_json::Value::String(s),
            Value::Blob(b) => {
                // Encode binary as base64 string
                serde_json::Value::String(base64_encode(&b))
            }
            Value::Boolean(b) => serde_json::Value::Bool(b),
        }
    }
}

fn base64_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for byte in data {
        write!(&mut s, "{:02x}", byte).unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_conversions() {
        let v: Value = 42i64.into();
        assert_eq!(v.as_i64().unwrap(), 42);

        let v: Value = "hello".into();
        assert_eq!(v.as_str().unwrap(), "hello");

        let v: Value = true.into();
        assert_eq!(v.as_bool().unwrap(), true);
    }

    #[test]
    fn test_json_conversion() {
        let json = serde_json::json!({
            "name": "Alice",
            "age": 30,
            "active": true
        });

        let name: Value = json["name"].clone().into();
        assert_eq!(name.as_str().unwrap(), "Alice");

        let age: Value = json["age"].clone().into();
        assert_eq!(age.as_i64().unwrap(), 30);
    }
}
