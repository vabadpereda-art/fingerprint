use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_uint};
use std::sync::{Arc, Mutex};

use nbis::{NbisExtractor, NbisExtractorSettings};
use serde_json::{self, Value as JsonValue};
use zkfp_db::{ColumnConstraint, DataType, SyncConfig, SyncEngine, Value, ZkfpDb};
use zkfp_image::{ContrastMethod, EnhanceConfig, GrayImage};
use zkfp_match::{Matcher, MemorySearchGallery};
use zkfp_usb::Zk9500;

#[repr(C)]
pub struct ZkfpEnhanceConfig {
    pub apply_enhancement: c_int,
    pub method: c_int,
    pub bg_intensity: c_uchar,
    pub invert: c_int,
    pub flip_vertical: c_int,
    pub padding: c_uint,
}

impl From<&ZkfpEnhanceConfig> for EnhanceConfig {
    fn from(c: &ZkfpEnhanceConfig) -> Self {
        Self {
            apply_enhancement: c.apply_enhancement != 0,
            method: if c.method == 1 {
                ContrastMethod::Darken
            } else {
                ContrastMethod::Stretch
            },
            bg_intensity: c.bg_intensity,
            invert: c.invert != 0,
            flip_vertical: c.flip_vertical != 0,
            padding: c.padding as u32,
        }
    }
}

impl From<&EnhanceConfig> for ZkfpEnhanceConfig {
    fn from(cfg: &EnhanceConfig) -> Self {
        Self {
            apply_enhancement: if cfg.apply_enhancement { 1 } else { 0 },
            method: match cfg.method {
                ContrastMethod::Stretch => 0,
                ContrastMethod::Darken => 1,
            },
            bg_intensity: cfg.bg_intensity,
            invert: if cfg.invert { 1 } else { 0 },
            flip_vertical: if cfg.flip_vertical { 1 } else { 0 },
            padding: cfg.padding as c_uint,
        }
    }
}

#[repr(C)]
pub struct ZkfpTemplate {
    pub data: *mut c_uchar,
    pub size: usize,
    pub quality: u32,
}

#[repr(C)]
pub struct ZkfpIdentifyVerifyResult {
    pub user_id: u32,
    pub identify_score: c_int,
    pub verify_score: c_int,
    pub identify_match: c_int,
    pub verify_match: c_int,
}

lazy_static::lazy_static! {
    pub static ref SCANNER: Mutex<Option<Zk9500>> = Mutex::new(None);
    pub static ref DATABASE: Mutex<Option<ZkfpDb>> = Mutex::new(None);
    pub static ref LAST_ERROR: Mutex<String> = Mutex::new(String::new());
    pub static ref ENHANCE_CONFIG: Mutex<EnhanceConfig> = Mutex::new(EnhanceConfig::default());
    pub static ref MATCHER: Mutex<Matcher> = Mutex::new(Matcher::with_default_threshold());
    pub static ref EXTRACTOR: Mutex<NbisExtractor> = Mutex::new(NbisExtractor::new(NbisExtractorSettings {
        min_quality: 0.0,
        get_center: false,
        check_fingerprint: false,
        compute_nfiq2: true,
        ppi: None,
    }).unwrap());
    pub static ref GALLERY: Mutex<MemorySearchGallery> = Mutex::new(MemorySearchGallery::new());
    pub static ref SYNC_CONFIG: Mutex<SyncConfig> = Mutex::new(SyncConfig::default());
    pub static ref SYNC_ENGINE: Mutex<Option<SyncEngine>> = Mutex::new(None);
    pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    pub static ref BULK_TX_ACTIVE: Mutex<bool> = Mutex::new(false);
    pub static ref BULK_TX_CONN: Mutex<Option<Arc<Mutex<rusqlite::Connection>>>> = Mutex::new(None);
    pub static ref BULK_ROW_BUFFER: Mutex<Vec<(String, Value)>> = Mutex::new(Vec::new());
}

pub fn set_error(err: &str) {
    if let Ok(mut lock) = LAST_ERROR.lock() {
        *lock = err.to_string();
    }
}

pub fn cstr_to_str<'a>(ptr: *const c_char, field: &str) -> Result<&'a str, ()> {
    if ptr.is_null() {
        set_error(&format!("{field} is null"));
        return Err(());
    }

    unsafe { CStr::from_ptr(ptr) }.to_str().map_err(|_| {
        set_error(&format!("{field} is not valid UTF-8"));
    })
}

pub fn with_db<T>(f: impl FnOnce(&ZkfpDb) -> Result<T, String>) -> Result<T, ()> {
    let guard = DATABASE.lock().map_err(|_| {
        set_error("Database mutex poisoned");
    })?;
    let db = guard.as_ref().ok_or_else(|| {
        set_error("Database not initialized");
    })?;
    f(db).map_err(|e| {
        set_error(&e);
    })
}

pub fn with_db_mut<T>(f: impl FnOnce(&mut ZkfpDb) -> Result<T, String>) -> Result<T, ()> {
    let mut guard = DATABASE.lock().map_err(|_| {
        set_error("Database mutex poisoned");
    })?;
    let db = guard.as_mut().ok_or_else(|| {
        set_error("Database not initialized");
    })?;
    f(db).map_err(|e| {
        set_error(&e);
    })
}

pub fn parse_data_type(code: c_int) -> Option<DataType> {
    match code {
        0 => Some(DataType::Integer),
        1 => Some(DataType::Real),
        2 => Some(DataType::Text),
        3 => Some(DataType::Blob),
        4 => Some(DataType::Boolean),
        5 => Some(DataType::Timestamp),
        _ => None,
    }
}

pub fn parse_constraints(
    flags: c_uint,
    foreign_table: Option<&str>,
    foreign_column: Option<&str>,
) -> Vec<ColumnConstraint> {
    let mut constraints = Vec::new();
    if flags & 0x01 != 0 {
        constraints.push(ColumnConstraint::PrimaryKey);
    }
    if flags & 0x02 != 0 {
        constraints.push(ColumnConstraint::NotNull);
    }
    if flags & 0x04 != 0 {
        constraints.push(ColumnConstraint::Unique);
    }
    if flags & 0x08 != 0 {
        constraints.push(ColumnConstraint::AutoIncrement);
    }
    if flags & 0x10 != 0 {
        if let (Some(table), Some(column)) = (foreign_table, foreign_column) {
            constraints.push(ColumnConstraint::ForeignKey {
                table: table.to_string(),
                column: column.to_string(),
            });
        }
    }
    constraints
}

pub fn json_value_from_c_parts(value_type: c_int, value: *const c_char) -> Result<JsonValue, ()> {
    if value.is_null() {
        return Ok(JsonValue::Null);
    }

    let raw = unsafe { CStr::from_ptr(value) }
        .to_str()
        .map_err(|_| set_error("Value is not valid UTF-8"))?;

    match value_type {
        0 => raw
            .parse::<i64>()
            .map(JsonValue::from)
            .map_err(|_| set_error("Invalid integer value")),
        1 => raw
            .parse::<f64>()
            .map(JsonValue::from)
            .map_err(|_| set_error("Invalid real value")),
        2 => Ok(JsonValue::String(raw.to_string())),
        3 => Ok(JsonValue::Bool(matches!(
            raw,
            "1" | "true" | "TRUE" | "True"
        ))),
        4 => Ok(JsonValue::Null),
        5 => serde_json::from_str(raw).map_err(|e| set_error(&format!("Invalid JSON value: {e}"))),
        _ => {
            set_error("Invalid value type");
            Err(())
        }
    }
}

pub fn load_enhanced_image_from_path(path: *const c_char) -> Result<GrayImage, ()> {
    let path_str = cstr_to_str(path, "path")?;
    let raw_img = GrayImage::from_file(path_str).map_err(|e| {
        set_error(&format!("Image decode error: {e}"));
    })?;
    Ok(raw_img.enhance_fingerprint())
}

pub fn write_c_string_out(out_ptr: *mut *mut c_char, value: String) -> c_int {
    if out_ptr.is_null() {
        set_error("Output string pointer is null");
        return 0;
    }
    match CString::new(value) {
        Ok(cs) => {
            unsafe {
                *out_ptr = cs.into_raw();
            }
            1
        }
        Err(_) => {
            set_error("Generated string contains interior NUL byte");
            unsafe {
                *out_ptr = std::ptr::null_mut();
            }
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_get_last_error() -> *const c_char {
    thread_local! {
        static C_ERROR: std::cell::RefCell<Option<CString>> = std::cell::RefCell::new(None);
    }

    let err_str = match LAST_ERROR.lock() {
        Ok(lock) => lock.clone(),
        Err(_) => "Mutex poisoned".to_string(),
    };

    C_ERROR.with(|cell| {
        let c_str = CString::new(err_str).unwrap_or_else(|_| CString::new("Unknown").unwrap());
        *cell.borrow_mut() = Some(c_str);
        cell.borrow().as_ref().unwrap().as_ptr()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_free_template(template: *mut ZkfpTemplate) {
    if !template.is_null() {
        unsafe {
            if !(*template).data.is_null() {
                let _ = Vec::from_raw_parts((*template).data, (*template).size, (*template).size);
                (*template).data = std::ptr::null_mut();
            }
        }
    }
}
