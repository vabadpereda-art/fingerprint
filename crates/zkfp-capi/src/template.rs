use std::os::raw::{c_char, c_int, c_uchar};

use zkfp_image::GrayImage;

use crate::common::{load_enhanced_image_from_path, set_error, write_c_string_out, EXTRACTOR, SCANNER, ZkfpTemplate};

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_extract_template(bmp_data: *const c_uchar, bmp_size: usize, out_template: *mut ZkfpTemplate) -> c_int {
    if bmp_data.is_null() || out_template.is_null() {
        set_error("Null pointer passed");
        return 0;
    }

    let slice = unsafe { std::slice::from_raw_parts(bmp_data, bmp_size) };
    let ext = match EXTRACTOR.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };

    match ext.extract_minutiae(slice) {
        Ok(minutiae) => {
            let mut iso_bytes = minutiae.to_iso_19794_2_2005();
            let size = iso_bytes.len();
            let quality = minutiae.quality().score as u32;
            let ptr = iso_bytes.as_mut_ptr();
            std::mem::forget(iso_bytes);
            unsafe {
                (*out_template).data = ptr;
                (*out_template).size = size;
                (*out_template).quality = quality;
            }
            1
        }
        Err(e) => {
            set_error(&format!("Extraction error: {e}"));
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_capture_full(out_template: *mut ZkfpTemplate, out_base64_png: *mut *mut c_char) -> c_int {
    if out_template.is_null() {
        set_error("out_template is null");
        return 0;
    }

    let mut scanner_guard = match SCANNER.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Scanner mutex poisoned");
            return 0;
        }
    };

    if let Some(dev) = scanner_guard.as_mut() {
        match dev.capture_image() {
            Ok(img) => {
                let raw_img = match GrayImage::from_raw(img.data, img.width, img.height) {
                    Ok(i) => i,
                    Err(e) => {
                        set_error(&format!("from_raw failed: {e}"));
                        return 0;
                    }
                };
                let enhanced = raw_img.enhance_fingerprint();

                if !out_base64_png.is_null() {
                    let _ = write_c_string_out(out_base64_png, enhanced.to_base64_png());
                }

                let bmp = enhanced.to_bmp();
                drop(scanner_guard);
                zkfp_extract_template(bmp.as_ptr(), bmp.len(), out_template)
            }
            Err(e) => {
                set_error(&format!("Capture error: {e}"));
                0
            }
        }
    } else {
        set_error("Scanner not initialized");
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_capture_and_extract_template(out_template: *mut ZkfpTemplate) -> c_int {
    zkfp_capture_full(out_template, std::ptr::null_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_extract_from_bmp_file(path: *const c_char, out_template: *mut ZkfpTemplate, out_base64_png: *mut *mut c_char) -> c_int {
    zkfp_extract_from_image_file(path, out_template, out_base64_png)
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_extract_from_image_file(path: *const c_char, out_template: *mut ZkfpTemplate, out_base64_png: *mut *mut c_char) -> c_int {
    if out_template.is_null() {
        set_error("Null pointer passed");
        return 0;
    }

    let enhanced = match load_enhanced_image_from_path(path) {
        Ok(img) => img,
        Err(_) => return 0,
    };

    if !out_base64_png.is_null() {
        let _ = write_c_string_out(out_base64_png, enhanced.to_base64_png());
    }

    let bmp = enhanced.to_bmp();
    zkfp_extract_template(bmp.as_ptr(), bmp.len(), out_template)
}
