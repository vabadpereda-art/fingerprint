use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};

use serde_json::{Map as JsonMap, Value as JsonValue};
use zkfp_db::{ColumnConstraint, DataType, Row, Value, ZkfpDb};

use crate::common::{
    BULK_ROW_BUFFER, BULK_TX_ACTIVE, BULK_TX_CONN, DATABASE, cstr_to_str, json_value_from_c_parts,
    parse_constraints, parse_data_type, set_error, with_db, with_db_mut,
};

fn row_to_json(row: Row) -> JsonValue {
    let mut obj = JsonMap::new();
    for (key, value) in row {
        obj.insert(key, db_value_to_json(value));
    }
    JsonValue::Object(obj)
}

fn db_value_to_json(value: Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Null,
        Value::Integer(i) => JsonValue::from(i),
        Value::Real(f) => JsonValue::from(f),
        Value::Text(s) => JsonValue::String(s),
        Value::Blob(bytes) => JsonValue::Array(bytes.into_iter().map(JsonValue::from).collect()),
        Value::Boolean(b) => JsonValue::from(b),
    }
}

fn json_string_ptr(value: &JsonValue) -> *mut c_char {
    match serde_json::to_string(value) {
        Ok(s) => match CString::new(s) {
            Ok(cs) => cs.into_raw(),
            Err(_) => {
                set_error("Generated JSON contains interior NUL byte");
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            set_error(&format!("Failed to serialize JSON: {e}"));
            std::ptr::null_mut()
        }
    }
}

fn parse_json_object(json_object: *const c_char) -> Result<JsonValue, ()> {
    let raw = cstr_to_str(json_object, "json_object")?;
    let value: JsonValue = serde_json::from_str(raw).map_err(|e| {
        set_error(&format!("Invalid JSON object: {e}"));
    })?;
    if !value.is_object() {
        set_error("json_object must be a JSON object");
        return Err(());
    }
    Ok(value)
}

fn typed_json_to_db_value(data_type: &DataType, value: &JsonValue) -> Result<Value, ()> {
    match data_type {
        DataType::Blob => match value {
            JsonValue::Array(items) => {
                let mut bytes = Vec::with_capacity(items.len());
                for item in items {
                    let Some(n) = item.as_u64() else {
                        set_error("Blob JSON arrays must contain only unsigned byte values");
                        return Err(());
                    };
                    if n > 255 {
                        set_error("Blob JSON arrays must contain values between 0 and 255");
                        return Err(());
                    }
                    bytes.push(n as u8);
                }
                Ok(Value::Blob(bytes))
            }
            JsonValue::Null => Ok(Value::Null),
            _ => {
                set_error("Blob columns require a JSON array of byte values");
                Err(())
            }
        },
        _ => Ok(value.clone().into()),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_open(db_path: *const c_char) -> c_int {
    let db_path = match cstr_to_str(db_path, "db_path") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match ZkfpDb::new(db_path) {
        Ok(db) => match DATABASE.lock() {
            Ok(mut guard) => {
                *guard = Some(db);
                1
            }
            Err(_) => {
                set_error("Database mutex poisoned");
                0
            }
        },
        Err(e) => {
            set_error(&format!("Failed to open database: {e}"));
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_close() {
    if let Ok(mut guard) = DATABASE.lock() {
        *guard = None;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_register_table(table_name: *const c_char) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match with_db_mut(|db| {
        db.register_table(table_name)
            .create()
            .map_err(|e| e.to_string())
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_create_fingerprint_schema(
    users_table: *const c_char,
    templates_table: *const c_char,
    user_name_column: *const c_char,
    template_user_id_column: *const c_char,
    template_finger_column: *const c_char,
    template_data_column: *const c_char,
    template_quality_column: *const c_char,
) -> c_int {
    let users_table = match cstr_to_str(users_table, "users_table") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let templates_table = match cstr_to_str(templates_table, "templates_table") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let user_name_column = match cstr_to_str(user_name_column, "user_name_column") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let template_user_id_column =
        match cstr_to_str(template_user_id_column, "template_user_id_column") {
            Ok(v) => v,
            Err(_) => return 0,
        };
    let template_finger_column = match cstr_to_str(template_finger_column, "template_finger_column")
    {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let template_data_column = match cstr_to_str(template_data_column, "template_data_column") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let template_quality_column =
        match cstr_to_str(template_quality_column, "template_quality_column") {
            Ok(v) => v,
            Err(_) => return 0,
        };

    let users_result = with_db_mut(|db| {
        db.register_table(users_table)
            .column(
                user_name_column,
                DataType::Text,
                vec![ColumnConstraint::NotNull],
            )
            .create()
            .map_err(|e| e.to_string())
    });

    if users_result.is_err() {
        return 0;
    }

    match with_db_mut(|db| {
        db.register_table(templates_table)
            .column(
                template_user_id_column,
                DataType::Integer,
                vec![ColumnConstraint::NotNull],
            )
            .column(
                template_finger_column,
                DataType::Integer,
                vec![ColumnConstraint::NotNull],
            )
            .column(
                template_data_column,
                DataType::Blob,
                vec![ColumnConstraint::NotNull],
            )
            .column(
                template_quality_column,
                DataType::Integer,
                vec![ColumnConstraint::NotNull],
            )
            .create()
            .map_err(|e| e.to_string())
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_add_column_and_create(
    table_name: *const c_char,
    column_name: *const c_char,
    data_type: c_int,
    constraint_flags: c_uint,
    foreign_table: *const c_char,
    foreign_column: *const c_char,
) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let column_name = match cstr_to_str(column_name, "column_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let data_type = match parse_data_type(data_type) {
        Some(v) => v,
        None => {
            set_error("Invalid data type");
            return 0;
        }
    };

    let foreign_table = if foreign_table.is_null() {
        None
    } else {
        cstr_to_str(foreign_table, "foreign_table").ok()
    };
    let foreign_column = if foreign_column.is_null() {
        None
    } else {
        cstr_to_str(foreign_column, "foreign_column").ok()
    };
    let constraints = parse_constraints(constraint_flags, foreign_table, foreign_column);

    match with_db_mut(|db| {
        db.register_table(table_name)
            .column(column_name, data_type, constraints)
            .create()
            .map_err(|e| e.to_string())
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_insert_kv(
    table_name: *const c_char,
    column_name: *const c_char,
    value_type: c_int,
    value: *const c_char,
    out_id: *mut i64,
) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let column_name = match cstr_to_str(column_name, "column_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let json_value = match json_value_from_c_parts(value_type, value) {
        Ok(v) => v,
        Err(_) => return 0,
    };

    let mut obj = JsonMap::new();
    obj.insert(column_name.to_string(), json_value);

    match with_db(|db| {
        db.insert(table_name, JsonValue::Object(obj))
            .map_err(|e| e.to_string())
    }) {
        Ok(id) => {
            if !out_id.is_null() {
                unsafe {
                    *out_id = id;
                }
            }
            1
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_insert_json(
    table_name: *const c_char,
    json_object: *const c_char,
    out_id: *mut i64,
) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let json_value = match parse_json_object(json_object) {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match with_db(|db| db.insert(table_name, json_value).map_err(|e| e.to_string())) {
        Ok(id) => {
            if !out_id.is_null() {
                unsafe {
                    *out_id = id;
                }
            }
            1
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_update_kv(
    table_name: *const c_char,
    row_id: i64,
    column_name: *const c_char,
    value_type: c_int,
    value: *const c_char,
) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let column_name = match cstr_to_str(column_name, "column_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let json_value = match json_value_from_c_parts(value_type, value) {
        Ok(v) => v,
        Err(_) => return 0,
    };

    let mut obj = JsonMap::new();
    obj.insert(column_name.to_string(), json_value);

    match with_db(|db| {
        db.update(table_name, row_id, JsonValue::Object(obj))
            .map_err(|e| e.to_string())
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_update_json(
    table_name: *const c_char,
    row_id: i64,
    json_object: *const c_char,
) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let json_value = match parse_json_object(json_object) {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match with_db(|db| {
        db.update(table_name, row_id, json_value)
            .map_err(|e| e.to_string())
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_delete_row(table_name: *const c_char, row_id: i64) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match with_db(|db| db.delete(table_name, row_id).map_err(|e| e.to_string())) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_get_row_json(table_name: *const c_char, row_id: i64) -> *mut c_char {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    match with_db(|db| {
        db.query(table_name)
            .where_eq("id", row_id)
            .fetch_one()
            .map_err(|e| e.to_string())
    }) {
        Ok(Some(row)) => json_string_ptr(&row_to_json(row)),
        Ok(None) => json_string_ptr(&JsonValue::Null),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_list_rows_json(table_name: *const c_char) -> *mut c_char {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    match with_db(|db| db.query(table_name).fetch_all().map_err(|e| e.to_string())) {
        Ok(rows) => {
            let json = JsonValue::Array(rows.into_iter().map(row_to_json).collect());
            json_string_ptr(&json)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_query_eq_json(
    table_name: *const c_char,
    column_name: *const c_char,
    value_type: c_int,
    value: *const c_char,
) -> *mut c_char {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    let column_name = match cstr_to_str(column_name, "column_name") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    let json_value = match json_value_from_c_parts(value_type, value) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    match with_db(|db| {
        db.query(table_name)
            .where_eq(column_name, json_value)
            .fetch_all()
            .map_err(|e| e.to_string())
    }) {
        Ok(rows) => {
            let json = JsonValue::Array(rows.into_iter().map(row_to_json).collect());
            json_string_ptr(&json)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_query_like_json(
    table_name: *const c_char,
    column_name: *const c_char,
    pattern: *const c_char,
) -> *mut c_char {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    let column_name = match cstr_to_str(column_name, "column_name") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };
    let pattern = match cstr_to_str(pattern, "pattern") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    match with_db(|db| {
        db.query(table_name)
            .where_like(column_name, pattern)
            .fetch_all()
            .map_err(|e| e.to_string())
    }) {
        Ok(rows) => {
            let json = JsonValue::Array(rows.into_iter().map(row_to_json).collect());
            json_string_ptr(&json)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_list_tables_json() -> *mut c_char {
    match with_db(|db| {
        let schema = db.local().get_table_schema("__nonexistent__").err();
        let _ = schema;
        let conn = db.local().connection();
        let conn = conn
            .lock()
            .map_err(|_| "Database connection mutex poisoned".to_string())?;
        let mut stmt = conn
            .prepare("SELECT table_name FROM _schema_registry ORDER BY table_name")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?;
        let mut tables = Vec::new();
        for row in rows {
            tables.push(JsonValue::String(row.map_err(|e| e.to_string())?));
        }
        Ok(JsonValue::Array(tables))
    }) {
        Ok(json) => json_string_ptr(&json),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_get_schema_json(table_name: *const c_char) -> *mut c_char {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    match with_db(|db| {
        let schema = db
            .local()
            .get_table_schema(table_name)
            .map_err(|e| e.to_string())?;
        serde_json::to_value(schema).map_err(|e| e.to_string())
    }) {
        Ok(json) => json_string_ptr(&json),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_count(table_name: *const c_char) -> i64 {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return -1,
    };

    match with_db(|db| db.query(table_name).count().map_err(|e| e.to_string())) {
        Ok(count) => count,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_delete_all_rows(table_name: *const c_char) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    match with_db(|db| {
        db.local()
            .get_table_schema(table_name)
            .map_err(|e| e.to_string())?;
        let conn = db.local().connection();
        let conn = conn
            .lock()
            .map_err(|_| "Database connection mutex poisoned".to_string())?;
        conn.execute(&format!("DELETE FROM {}", table_name), [])
            .map_err(|e| e.to_string())?;
        Ok(())
    }) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_bulk_begin() -> c_int {
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

    let conn = db.local().connection();
    {
        let conn_lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => {
                set_error("Database connection mutex poisoned");
                return 0;
            }
        };
        if let Err(e) = conn_lock.execute_batch(
            "PRAGMA journal_mode = MEMORY;
             PRAGMA synchronous = OFF;
             PRAGMA temp_store = MEMORY;
             BEGIN IMMEDIATE TRANSACTION;",
        ) {
            set_error(&format!("Failed to begin bulk transaction: {e}"));
            return 0;
        }
    }

    match BULK_TX_ACTIVE.lock() {
        Ok(mut active) => *active = true,
        Err(_) => {
            set_error("Bulk transaction state mutex poisoned");
            return 0;
        }
    }
    match BULK_TX_CONN.lock() {
        Ok(mut slot) => *slot = Some(conn),
        Err(_) => {
            set_error("Bulk transaction connection mutex poisoned");
            return 0;
        }
    }

    1
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_bulk_insert_json(
    table_name: *const c_char,
    json_object: *const c_char,
    out_id: *mut i64,
) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let json_value = match parse_json_object(json_object) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let obj = match json_value.as_object() {
        Some(v) => v,
        None => {
            set_error("json_object must be a JSON object");
            return 0;
        }
    };

    let conn_arc = match BULK_TX_CONN.lock() {
        Ok(slot) => match slot.as_ref() {
            Some(conn) => conn.clone(),
            None => {
                set_error("Bulk transaction not started");
                return 0;
            }
        },
        Err(_) => {
            set_error("Bulk transaction connection mutex poisoned");
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
    let table_schema = match db.local().get_table_schema(table_name) {
        Ok(schema) => schema,
        Err(e) => {
            set_error(&e.to_string());
            return 0;
        }
    };

    let mut columns = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    for (key, val) in obj {
        if key.starts_with('_') {
            continue;
        }
        let Some(column) = table_schema.get_column(key) else {
            set_error(&format!("Column not found: {key}"));
            return 0;
        };
        let typed_value = match typed_json_to_db_value(&column.data_type, val) {
            Ok(v) => v,
            Err(_) => return 0,
        };
        columns.push(key.clone());
        values.push(typed_value);
    }

    let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("?{}", i)).collect();
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        columns.join(", "),
        placeholders.join(", ")
    );

    let conn_lock = match conn_arc.lock() {
        Ok(lock) => lock,
        Err(_) => {
            set_error("Database connection mutex poisoned");
            return 0;
        }
    };
    let params: Vec<&dyn rusqlite::ToSql> =
        values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    match conn_lock.execute(&sql, params.as_slice()) {
        Ok(_) => {
            if !out_id.is_null() {
                unsafe {
                    *out_id = conn_lock.last_insert_rowid();
                }
            }
            1
        }
        Err(e) => {
            set_error(&format!("Bulk insert failed: {e}"));
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_bulk_row_begin() -> c_int {
    match BULK_ROW_BUFFER.lock() {
        Ok(mut row) => {
            row.clear();
            1
        }
        Err(_) => {
            set_error("Bulk row buffer mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_bulk_row_add_value(
    column_name: *const c_char,
    value_type: c_int,
    value: *const c_char,
    blob_data: *const u8,
    blob_size: usize,
) -> c_int {
    let column_name = match cstr_to_str(column_name, "column_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    let typed_value = if value_type == 3 {
        if blob_data.is_null() && blob_size > 0 {
            set_error("blob_data is null");
            return 0;
        }
        let bytes = if blob_size == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(blob_data, blob_size) }.to_vec()
        };
        Value::Blob(bytes)
    } else {
        let json_value = match json_value_from_c_parts(value_type, value) {
            Ok(v) => v,
            Err(_) => return 0,
        };
        json_value.into()
    };

    match BULK_ROW_BUFFER.lock() {
        Ok(mut row) => {
            row.push((column_name.to_string(), typed_value));
            1
        }
        Err(_) => {
            set_error("Bulk row buffer mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_bulk_row_insert(table_name: *const c_char, out_id: *mut i64) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };

    let conn_arc = match BULK_TX_CONN.lock() {
        Ok(slot) => match slot.as_ref() {
            Some(conn) => conn.clone(),
            None => {
                set_error("Bulk transaction not started");
                return 0;
            }
        },
        Err(_) => {
            set_error("Bulk transaction connection mutex poisoned");
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
    let table_schema = match db.local().get_table_schema(table_name) {
        Ok(schema) => schema,
        Err(e) => {
            set_error(&e.to_string());
            return 0;
        }
    };

    let row_values = match BULK_ROW_BUFFER.lock() {
        Ok(row) => row.clone(),
        Err(_) => {
            set_error("Bulk row buffer mutex poisoned");
            return 0;
        }
    };

    let mut columns = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    for (key, typed_value) in row_values {
        if key.starts_with('_') {
            continue;
        }
        if table_schema.get_column(&key).is_none() {
            set_error(&format!("Column not found: {key}"));
            return 0;
        }
        columns.push(key);
        values.push(typed_value);
    }

    let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("?{}", i)).collect();
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        columns.join(", "),
        placeholders.join(", ")
    );

    let conn_lock = match conn_arc.lock() {
        Ok(lock) => lock,
        Err(_) => {
            set_error("Database connection mutex poisoned");
            return 0;
        }
    };
    let params: Vec<&dyn rusqlite::ToSql> =
        values.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
    match conn_lock.execute(&sql, params.as_slice()) {
        Ok(_) => {
            if !out_id.is_null() {
                unsafe {
                    *out_id = conn_lock.last_insert_rowid();
                }
            }
            1
        }
        Err(e) => {
            set_error(&format!("Typed bulk insert failed: {e}"));
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_bulk_commit() -> c_int {
    let conn_arc = match BULK_TX_CONN.lock() {
        Ok(mut slot) => match slot.take() {
            Some(conn) => conn,
            None => {
                set_error("Bulk transaction not started");
                return 0;
            }
        },
        Err(_) => {
            set_error("Bulk transaction connection mutex poisoned");
            return 0;
        }
    };

    let conn_lock = match conn_arc.lock() {
        Ok(lock) => lock,
        Err(_) => {
            set_error("Database connection mutex poisoned");
            return 0;
        }
    };
    if let Err(e) = conn_lock.execute("COMMIT", []) {
        set_error(&format!("Failed to commit bulk transaction: {e}"));
        return 0;
    }
    if let Ok(mut active) = BULK_TX_ACTIVE.lock() {
        *active = false;
    }
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_db_bulk_rollback() -> c_int {
    let conn_arc = match BULK_TX_CONN.lock() {
        Ok(mut slot) => match slot.take() {
            Some(conn) => conn,
            None => {
                set_error("Bulk transaction not started");
                return 0;
            }
        },
        Err(_) => {
            set_error("Bulk transaction connection mutex poisoned");
            return 0;
        }
    };

    let conn_lock = match conn_arc.lock() {
        Ok(lock) => lock,
        Err(_) => {
            set_error("Database connection mutex poisoned");
            return 0;
        }
    };
    if let Err(e) = conn_lock.execute("ROLLBACK", []) {
        set_error(&format!("Failed to rollback bulk transaction: {e}"));
        return 0;
    }
    if let Ok(mut active) = BULK_TX_ACTIVE.lock() {
        *active = false;
    }
    1
}
