use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint};

use zkfp_image::{ContrastMethod, EnhanceConfig, GrayImage};

use crate::common::{
    ENHANCE_CONFIG, SCANNER, ZkfpEnhanceConfig, cstr_to_str, load_enhanced_image_from_path,
    set_error, write_c_string_out,
};

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_capture_image_base64(format: *const c_char) -> *mut c_char {
    let format_str = match cstr_to_str(format, "format") {
        Ok(v) => v.to_lowercase(),
        Err(_) => return std::ptr::null_mut(),
    };

    let mut scanner_guard = match SCANNER.lock() {
        Ok(guard) => guard,
        Err(_) => return std::ptr::null_mut(),
    };

    if let Some(dev) = scanner_guard.as_mut() {
        match dev.capture_image() {
            Ok(img) => {
                let raw_img = match GrayImage::from_raw(img.data, img.width, img.height) {
                    Ok(img) => img,
                    Err(_) => {
                        set_error("Failed to create GrayImage");
                        return std::ptr::null_mut();
                    }
                };

                let config = match ENHANCE_CONFIG.lock() {
                    Ok(cfg) => cfg.clone(),
                    Err(_) => EnhanceConfig::default(),
                };
                let enhanced = raw_img.enhance_with_config(config);

                match enhanced.to_base64_with_format(&format_str) {
                    Ok(value) => CString::new(value)
                        .map(|c| c.into_raw())
                        .unwrap_or(std::ptr::null_mut()),
                    Err(e) => {
                        set_error(&format!(
                            "Unsupported or failed output format '{format_str}': {e}"
                        ));
                        std::ptr::null_mut()
                    }
                }
            }
            Err(e) => {
                set_error(&format!("Capture failed: {e:?}"));
                std::ptr::null_mut()
            }
        }
    } else {
        set_error("Scanner not initialized");
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_image_file_to_base64(
    path: *const c_char,
    format: *const c_char,
    out_base64: *mut *mut c_char,
) -> c_int {
    let format_str = match cstr_to_str(format, "format") {
        Ok(s) => s.to_lowercase(),
        Err(_) => return 0,
    };

    let enhanced = match load_enhanced_image_from_path(path) {
        Ok(img) => img,
        Err(_) => return 0,
    };

    match enhanced.to_base64_with_format(&format_str) {
        Ok(b64) => write_c_string_out(out_base64, b64),
        Err(e) => {
            set_error(&format!("Failed to export image as '{format_str}': {e}"));
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_set_enhance_config(config: *const ZkfpEnhanceConfig) -> c_int {
    if config.is_null() {
        set_error("Config pointer is null");
        return 0;
    }
    let c_config = unsafe { &*config };
    let rust_config = EnhanceConfig::from(c_config);
    match ENHANCE_CONFIG.lock() {
        Ok(mut guard) => {
            *guard = rust_config;
            1
        }
        Err(_) => {
            set_error("Config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_get_enhance_config(config: *mut ZkfpEnhanceConfig) -> c_int {
    if config.is_null() {
        set_error("Config pointer is null");
        return 0;
    }
    match ENHANCE_CONFIG.lock() {
        Ok(guard) => {
            let c_cfg = ZkfpEnhanceConfig::from(&*guard);
            unsafe {
                std::ptr::write(config, c_cfg);
            }
            1
        }
        Err(_) => {
            set_error("Config mutex poisoned");
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_set_contrast_method(method: c_int) -> c_int {
    match ENHANCE_CONFIG.lock() {
        Ok(mut guard) => {
            guard.method = if method == 1 {
                ContrastMethod::Darken
            } else {
                ContrastMethod::Stretch
            };
            1
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_set_invert(invert: c_int) -> c_int {
    match ENHANCE_CONFIG.lock() {
        Ok(mut guard) => {
            guard.invert = invert != 0;
            1
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_set_flip_vertical(flip: c_int) -> c_int {
    match ENHANCE_CONFIG.lock() {
        Ok(mut guard) => {
            guard.flip_vertical = flip != 0;
            1
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_set_bg_intensity(intensity: c_uchar) -> c_int {
    match ENHANCE_CONFIG.lock() {
        Ok(mut guard) => {
            guard.bg_intensity = intensity;
            1
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_set_padding(padding: c_uint) -> c_int {
    match ENHANCE_CONFIG.lock() {
        Ok(mut guard) => {
            guard.padding = padding as u32;
            1
        }
        Err(_) => 0,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_set_enhancement_enabled(enabled: c_int) -> c_int {
    match ENHANCE_CONFIG.lock() {
        Ok(mut guard) => {
            guard.apply_enhancement = enabled != 0;
            1
        }
        Err(_) => 0,
    }
}
