use std::os::raw::c_int;

use zkfp_usb::{LedColor, Zk9500};

use crate::common::{SCANNER, set_error};

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_init() -> c_int {
    let mut scanner_guard = match SCANNER.lock() {
        Ok(guard) => guard,
        Err(_) => {
            set_error("Scanner mutex poisoned");
            return 0;
        }
    };

    if scanner_guard.is_some() {
        return 1;
    }

    match Zk9500::open() {
        Ok(mut dev) => match dev.init() {
            Ok(_) => {
                let _ = dev.set_led(LedColor::Green, true);
                *scanner_guard = Some(dev);
                1
            }
            Err(e) => {
                set_error(&format!("Init failed: {e:?}"));
                0
            }
        },
        Err(e) => {
            set_error(&format!("Open failed: {e:?}"));
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn zkfp_close() {
    if let Ok(mut scanner_guard) = SCANNER.lock() {
        if let Some(dev) = scanner_guard.take() {
            let _ = dev.led_off();
        }
    }
}
