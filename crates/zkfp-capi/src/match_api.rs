use std::os::raw::{c_char, c_int, c_uchar};

use zkfp_template::FingerprintTemplate;

use crate::common::{DATABASE, GALLERY, MATCHER, ZkfpIdentifyVerifyResult, cstr_to_str, set_error};

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_verify_templates(
    tmpl1_data: *const c_uchar,
    tmpl1_size: usize,
    tmpl2_data: *const c_uchar,
    tmpl2_size: usize,
) -> c_int {
    let t1_bytes = unsafe { std::slice::from_raw_parts(tmpl1_data, tmpl1_size) }.to_vec();
    let t2_bytes = unsafe { std::slice::from_raw_parts(tmpl2_data, tmpl2_size) }.to_vec();

    let t1 = FingerprintTemplate {
        iso_bytes: t1_bytes,
        quality: 50,
    };
    let t2 = FingerprintTemplate {
        iso_bytes: t2_bytes,
        quality: 50,
    };

    let matcher = match MATCHER.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };

    matcher.verify(&t1, &t2).score as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_gallery_clear() {
    if let Ok(mut g) = GALLERY.lock() {
        g.clear();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_gallery_add(
    user_id: u32,
    tmpl_data: *const c_uchar,
    tmpl_size: usize,
) -> c_int {
    let t_bytes = unsafe { std::slice::from_raw_parts(tmpl_data, tmpl_size) }.to_vec();
    let t = FingerprintTemplate {
        iso_bytes: t_bytes,
        quality: 50,
    };

    let matcher = match MATCHER.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };

    if let Ok(mut g) = GALLERY.lock() {
        let search_template = matcher.create_search_template(&t);
        g.insert(user_id, search_template);
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_gallery_load_from_db(
    table_name: *const c_char,
    user_id_column: *const c_char,
    template_column: *const c_char,
) -> c_int {
    let table_name = match cstr_to_str(table_name, "table_name") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let user_id_column = match cstr_to_str(user_id_column, "user_id_column") {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let template_column = match cstr_to_str(template_column, "template_column") {
        Ok(v) => v,
        Err(_) => return 0,
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

    let rows = match db.query(table_name).fetch_all() {
        Ok(rows) => rows,
        Err(e) => {
            set_error(&format!("Failed to read local templates: {e}"));
            return 0;
        }
    };

    let matcher = match MATCHER.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Matcher mutex poisoned");
            return 0;
        }
    };

    let mut gallery = match GALLERY.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Gallery mutex poisoned");
            return 0;
        }
    };

    gallery.clear();
    let mut loaded = 0u32;
    for row in rows {
        let user_id = match row.get(user_id_column) {
            Some(zkfp_db::Value::Integer(v)) => *v as u32,
            Some(zkfp_db::Value::Text(v)) => match v.parse::<u32>() {
                Ok(parsed) => parsed,
                Err(_) => continue,
            },
            _ => continue,
        };
        let tmpl_bytes = match row.get(template_column) {
            Some(zkfp_db::Value::Blob(bytes)) => bytes.clone(),
            _ => continue,
        };

        let template = FingerprintTemplate {
            iso_bytes: tmpl_bytes,
            quality: 50,
        };
        let search_template = matcher.create_search_template(&template);
        gallery.insert(user_id, search_template);
        loaded += 1;
    }

    loaded as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_gallery_remove(user_id: u32) {
    if let Ok(mut g) = GALLERY.lock() {
        g.remove(user_id);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_gallery_identify(
    probe_data: *const c_uchar,
    probe_size: usize,
    out_id: *mut u32,
    out_score: *mut c_int,
) -> c_int {
    let probe_bytes = unsafe { std::slice::from_raw_parts(probe_data, probe_size) }.to_vec();
    let probe = FingerprintTemplate {
        iso_bytes: probe_bytes,
        quality: 50,
    };

    let matcher = match MATCHER.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };

    let g = match GALLERY.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };

    let result = matcher.identify_in_memory(&probe, &g);
    if result.score >= matcher.threshold() {
        if !out_id.is_null() && !out_score.is_null() {
            unsafe {
                *out_id = result.user_id.unwrap_or(0);
                *out_score = result.score as c_int;
            }
        }
        1
    } else {
        if !out_score.is_null() {
            unsafe {
                *out_score = result.score as c_int;
            }
        }
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_gallery_identify_with_verification(
    probe_data: *const c_uchar,
    probe_size: usize,
    out_result: *mut ZkfpIdentifyVerifyResult,
) -> c_int {
    if probe_data.is_null() || out_result.is_null() {
        set_error("Null pointer passed");
        return 0;
    }

    let probe_bytes = unsafe { std::slice::from_raw_parts(probe_data, probe_size) }.to_vec();
    let probe = FingerprintTemplate {
        iso_bytes: probe_bytes,
        quality: 50,
    };

    let matcher = match MATCHER.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Matcher mutex poisoned");
            return 0;
        }
    };

    let g = match GALLERY.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Gallery mutex poisoned");
            return 0;
        }
    };

    let result = matcher.identify_in_memory_with_verification(&probe, &g);
    let identify_match =
        if result.identify.score >= matcher.threshold() && result.identify.user_id.is_some() {
            1
        } else {
            0
        };
    let verify_match = if result.verify.score >= matcher.verify_threshold() {
        1
    } else {
        0
    };

    unsafe {
        (*out_result).user_id = result.identify.user_id.unwrap_or(0);
        (*out_result).identify_score = result.identify.score as c_int;
        (*out_result).verify_score = result.verify.score as c_int;
        (*out_result).identify_match = identify_match;
        (*out_result).verify_match = verify_match;
    }

    if identify_match != 0 && verify_match != 0 {
        1
    } else {
        0
    }
}
