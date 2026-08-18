//! # zkfp-usb
//!
//! USB communication with ZKTeco ZK9500 fingerprint scanner.
//!
//! Device: VID `0x1B55`, PID `0x0124`
//! Optical scanner, 500 DPI, 2MP, USB 2.0
//!
//! ## USB Protocol (reverse-engineered from `zkandroidfpreader.jar` + `libzkfinger10.so`)
//!
//! The ZK9500 uses **USB control transfers** for commands and **bulk transfers**
//! for image data. Reverse-engineered from the decompiled Java SDK
//! (`FingerprintSensor.class`) and native `.so`.
//!
//! ### Endpoints (from USB descriptor)
//! - **EP 1 IN** (0x81): Interrupt, 64 bytes — finger detection status
//! - **EP 2 IN** (0x82): Bulk, 512 bytes — image data read
//! - **EP 3 OUT** (0x03): Bulk, 512 bytes — reserved
//!
//! ### Command Protocol (USB Control Transfers)
//! All commands use vendor-type control transfers on endpoint 0:
//! - **OUT commands**: `bmRequestType = 0x40` (vendor, device, OUT)
//! - **IN commands**: `bmRequestType = 0xC0` (vendor, device, IN)
//! - `bRequest` = command code (see CMD_* constants)
//! - `wValue` / `wIndex` = command-specific parameters
//!
//! ### Init Sequence
//! 1. Open device, claim interface 0
//! 2. `CMD_INIT` (0xE0) control OUT → reset device
//! 3. Sleep 100ms
//! 4. Handshake via `CMD_GET_GPIO` (0xE2) with index=0x55 (GET_VERSION)
//! 5. Read firmware version (2 bytes)
//!
//! ### Image Capture Sequence
//! 1. Set work mode via `CMD_SET_GPIO` (0xE1)
//! 2. `CMD_GET_IMAGE` (0xE5) control OUT → start capture
//! 3. Read 112500 bytes (300×375) via bulk IN (EP 0x82) in 16384-byte chunks
//! 4. Assemble into grayscale image

use std::time::Duration;

use rusb::UsbContext;
use thiserror::Error;

// --- USB Identifiers ---

/// USB Vendor ID for ZKTeco devices
pub const ZKTECO_VID: u16 = 0x1B55;

/// USB Product ID for ZK9500
pub const ZK9500_PID: u16 = 0x0124;

// --- Image Constants ---

/// Expected image width (ZK9500 native resolution)
pub const DEFAULT_IMAGE_WIDTH: u32 = 300;

/// Expected image height
pub const DEFAULT_IMAGE_HEIGHT: u32 = 375;

/// Expected DPI
pub const DEFAULT_DPI: u32 = 500;

/// Total image size in bytes (300 × 375 = 112500)
pub const IMAGE_SIZE: usize = (DEFAULT_IMAGE_WIDTH * DEFAULT_IMAGE_HEIGHT) as usize;

// --- USB Endpoints (from USB descriptor) ---

/// USB interface number used by the scanner
pub const USB_INTERFACE: u8 = 0;

/// Interrupt IN endpoint — finger detection status
pub const INTERRUPT_IN_ENDPOINT: u8 = 0x81;

/// Bulk IN endpoint — image data read
pub const BULK_IN_ENDPOINT: u8 = 0x82;

/// Bulk OUT endpoint — reserved
pub const BULK_OUT_ENDPOINT: u8 = 0x03;

/// Maximum bulk transfer chunk size (from SDK: 16384)
pub const BULK_CHUNK_SIZE: usize = 16384;

/// Maximum bulk packet size (USB 2.0 high-speed)
pub const BULK_MAX_PACKET: usize = 512;

// --- USB Control Transfer Request Types ---

/// Vendor-type OUT control transfer (bmRequestType)
pub const CTRL_OUT: u8 = 0x40;

/// Vendor-type IN control transfer (bmRequestType)
pub const CTRL_IN: u8 = 0xC0;

// --- Command Codes (from decompiled FingerprintSensor.class) ---

/// Initialize / reset device
pub const CMD_INIT: u8 = 0xE0;

/// Handshake command (sends bulk response)
pub const CMD_HANDSHAKE: u8 = 0x80;

/// Set GPIO register (LED, mode, etc.)
pub const CMD_SET_GPIO: u8 = 0xE1;

/// Get GPIO register (version, status, etc.)
pub const CMD_GET_GPIO: u8 = 0xE2;

/// Write CMOS/camera register
pub const CMD_SET_CMOS: u8 = 0xE3;

/// Read CMOS/camera register
pub const CMD_GET_CMOS: u8 = 0xE4;

/// Start image capture (triggers bulk IN transfer)
pub const CMD_GET_IMAGE: u8 = 0xE5;

/// Write non-volatile memory
pub const CMD_SET_NVM: u8 = 0xE6;

/// Read non-volatile memory
pub const CMD_GET_NVM: u8 = 0xE7;

/// Write I2C register
pub const CMD_SET_IIC: u8 = 0xE8;

/// Read I2C register
pub const CMD_GET_IIC: u8 = 0xE9;

/// Detect image (finger presence check)
pub const CMD_DET_IMAGE: u8 = 0xEA;

/// Clear image buffer
pub const CMD_CLR_IMAGE: u8 = 0xEB;

/// Check license
pub const CMD_CHECK_LIC: u8 = 0xF3;

// --- GPIO Register Indices (for CMD_SET_GPIO / CMD_GET_GPIO) ---

/// Get firmware version (index for CMD_GET_GPIO)
pub const GPIO_GET_VERSION: u16 = 0x55;

/// Work mode register
pub const GPIO_WORK_MODE: u16 = 0x30;

/// Output mode register
pub const GPIO_OUTPUT_MODE: u16 = 0x31;

/// LED register (SILKID_LED_ADDR = 112 = 0x70)
pub const GPIO_LED: u16 = 0x70;

/// Anti-fake register
pub const GPIO_ANTI_FAKE: u16 = 0x32;

/// Power mode register
pub const GPIO_POWER_MODE: u16 = 0x53;

/// Sensor capture area
pub const GPIO_IMAGE_AREA_6: u16 = 0x06;
pub const GPIO_IMAGE_AREA_7: u16 = 0x07;

// --- Work Mode Values (for CMD_SET_GPIO with GPIO_WORK_MODE) ---

/// Idle mode
pub const MODE_IDLE: u16 = 0;

/// Main capture mode
pub const MODE_MAIN: u16 = 1;

/// Fake detection mode
pub const MODE_FAKE: u16 = 2;

/// Detect mode (finger presence)
pub const MODE_DETECT: u16 = 3;

// --- LED Values (for CMD_SET_GPIO with GPIO_LED) ---

/// Close all LEDs
pub const LED_ALL_OFF: u16 = 0;

/// Open main (green) LED
pub const LED_MAIN_ON: u16 = 15;

/// Open fake detection LED
pub const LED_FAKE_ON: u16 = 16;

// --- CMOS Parameters (for CMD_SET_CMOS) ---

/// CMOS sensor width
pub const CMOS_WIDTH: u16 = 1600;

/// CMOS sensor height
pub const CMOS_HEIGHT: u16 = 1200;

// --- Timeouts ---

/// USB timeout for control transfers (ms)
pub const CTRL_TIMEOUT: Duration = Duration::from_millis(500);

/// USB timeout for bulk reads during image capture (ms)
pub const BULK_TIMEOUT: Duration = Duration::from_millis(1000);

/// USB timeout for interrupt reads (finger detection) (ms)
pub const INTERRUPT_TIMEOUT: Duration = Duration::from_millis(100);

/// Post-init stabilization delay
pub const INIT_DELAY: Duration = Duration::from_millis(100);

/// Maximum capture attempts before timeout
pub const MAX_CAPTURE_ATTEMPTS: u32 = 30; // 15s / 100ms

// --- Error types ---

#[derive(Error, Debug)]
pub enum UsbError {
    #[error("Device not found: VID={vid:#06x} PID={pid:#06x}")]
    DeviceNotFound { vid: u16, pid: u16 },

    #[error("USB access denied (try running as root or add udev rules)")]
    AccessDenied,

    #[error("USB I/O error: {0}")]
    Io(#[from] rusb::Error),

    #[error("Capture timeout: no finger detected within {0}ms")]
    CaptureTimeout(u64),

    #[error("Invalid image size: expected {expected} bytes, got {actual}")]
    InvalidImageSize { expected: usize, actual: usize },

    #[error("Invalid image dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },

    #[error("Device busy")]
    DeviceBusy,

    #[error("Firmware error: {0}")]
    Firmware(String),

    #[error("Control transfer failed: command={command:#04x}, reason={reason}")]
    ControlFailed { command: u8, reason: String },

    #[error("Device not initialized")]
    NotInitialized,
}

// --- Types ---

/// Raw fingerprint image captured from the scanner
#[derive(Clone, Debug)]
pub struct RawImage {
    pub width: u32,
    pub height: u32,
    pub dpi: u32,
    pub data: Vec<u8>,
}

impl RawImage {
    pub fn new(width: u32, height: u32, dpi: u32, data: Vec<u8>) -> Result<Self, UsbError> {
        let expected = (width * height) as usize;
        if data.len() != expected {
            return Err(UsbError::InvalidDimensions { width, height });
        }
        Ok(Self { width, height, dpi, data })
    }

    pub fn default_size(data: Vec<u8>) -> Result<Self, UsbError> {
        Self::new(DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT, DEFAULT_DPI, data)
    }

    pub fn pixel(&self, x: u32, y: u32) -> u8 {
        self.data[(y * self.width + x) as usize]
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// LED color on the ZK9500
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedColor {
    Green,
    Red,
    White,
}

/// Device information discovered on USB bus
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub vid: u16,
    pub pid: u16,
    pub manufacturer: String,
    pub product: String,
    pub serial: Option<String>,
    pub bus_number: u8,
    pub address: u8,
}

/// Firmware version info
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirmwareVersion {
    pub major: u8,
    pub minor: u8,
}

impl std::fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

// --- Device enumeration ---

/// List all ZKTeco fingerprint scanners on the USB bus
pub fn list_devices() -> Result<Vec<DeviceInfo>, UsbError> {
    let context = rusb::Context::new()?;
    let mut devices = Vec::new();

    for device in context.devices()?.iter() {
        let desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if desc.vendor_id() == ZKTECO_VID {
            let manufacturer = match device.open() {
                Ok(h) => h.read_manufacturer_string_ascii(&desc).unwrap_or_default(),
                Err(_) => String::new(),
            };
            let product = match device.open() {
                Ok(h) => h.read_product_string_ascii(&desc).unwrap_or_default(),
                Err(_) => String::new(),
            };
            let serial = match device.open() {
                Ok(h) => h.read_serial_number_string_ascii(&desc).ok(),
                Err(_) => None,
            };

            devices.push(DeviceInfo {
                vid: desc.vendor_id(),
                pid: desc.product_id(),
                manufacturer,
                product,
                serial,
                bus_number: device.bus_number(),
                address: device.address(),
            });
        }
    }

    Ok(devices)
}

/// Check if a ZK9500 device is present on the USB bus
pub fn is_device_present() -> bool {
    let context = match rusb::Context::new() {
        Ok(c) => c,
        Err(_) => return false,
    };
    context.open_device_with_vid_pid(ZKTECO_VID, ZK9500_PID).is_some()
}

// --- Device ---

/// ZK9500 fingerprint scanner handle
///
/// Implements the full USB protocol reverse-engineered from the ZKTeco SDK.
/// All commands use USB control transfers (vendor type), image data uses
/// bulk IN transfers on endpoint 0x82.
pub struct Zk9500 {
    handle: rusb::DeviceHandle<rusb::Context>,
    initialized: bool,
}

impl Zk9500 {
    /// Open the first ZK9500 device found on the USB bus
    pub fn open() -> Result<Self, UsbError> {
        Self::open_with_vid_pid(ZKTECO_VID, ZK9500_PID)
    }

    /// Open device by specific VID/PID
    pub fn open_with_vid_pid(vid: u16, pid: u16) -> Result<Self, UsbError> {
        let context = rusb::Context::new()?;
        let handle = context.open_device_with_vid_pid(vid, pid)
            .ok_or(UsbError::DeviceNotFound { vid, pid })?;

        // Try to claim interface 0
        if handle.claim_interface(USB_INTERFACE).is_err() {
            if handle.kernel_driver_active(USB_INTERFACE).unwrap_or(false) {
                handle.detach_kernel_driver(USB_INTERFACE)?;
            }
            handle.claim_interface(USB_INTERFACE)?;
        }

        Ok(Self { handle, initialized: false })
    }

    // --- Low-level USB transfers ---

    /// Send a vendor-type OUT control transfer (command to device)
    pub fn control_out(&self, request: u8, value: u16, index: u16) -> Result<(), UsbError> {
        self.handle.write_control(
            CTRL_OUT,
            request,
            value,
            index,
            &[],
            CTRL_TIMEOUT,
        )?;
        Ok(())
    }

    /// Send a vendor-type IN control transfer (read from device)
    fn control_in(&self, request: u8, value: u16, index: u16, buf: &mut [u8]) -> Result<usize, UsbError> {
        let read = self.handle.read_control(
            CTRL_IN,
            request,
            value,
            index,
            buf,
            CTRL_TIMEOUT,
        )?;
        Ok(read)
    }

    /// Read data from bulk IN endpoint (image data)
    pub fn bulk_read(&self, buf: &mut [u8]) -> Result<usize, UsbError> {
        let read = self.handle.read_bulk(
            BULK_IN_ENDPOINT,
            buf,
            BULK_TIMEOUT,
        )?;
        Ok(read)
    }

    /// Read from interrupt IN endpoint (finger detection)
    #[allow(dead_code)]
    fn interrupt_read(&self, buf: &mut [u8]) -> Result<usize, UsbError> {
        let read = self.handle.read_interrupt(
            INTERRUPT_IN_ENDPOINT,
            buf,
            INTERRUPT_TIMEOUT,
        )?;
        Ok(read)
    }

    /// Drain any pending data from bulk and interrupt endpoints
    pub fn drain_buffers(&self) {
        let mut drain_buf = [0u8; BULK_CHUNK_SIZE]; // 16384 bytes
        // Drain bulk IN
        for _ in 0..10 {
            match self.handle.read_bulk(BULK_IN_ENDPOINT, &mut drain_buf, Duration::from_millis(100)) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }
        // Drain interrupt IN
        let mut int_buf = [0u8; 64];
        for _ in 0..10 {
            match self.handle.read_interrupt(INTERRUPT_IN_ENDPOINT, &mut int_buf, Duration::from_millis(10)) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }
    }

    // --- GPIO / Register Access ---

    /// Set GPIO register (CMD_SET_GPIO = 0xE1)
    ///
    /// Maps to: `controlEx(0x40, 0xE1, value, index, null, 0, 500)`
    pub fn set_gpio(&self, index: u16, value: u16) -> Result<(), UsbError> {
        self.control_out(CMD_SET_GPIO, value, index)
    }

    /// Get GPIO register (CMD_GET_GPIO = 0xE2)
    ///
    /// Maps to: `controlEx(0xC0, 0xE2, 0, index, data, length, 500)`
    pub fn get_gpio(&self, index: u16, buf: &mut [u8]) -> Result<usize, UsbError> {
        self.control_in(CMD_GET_GPIO, 0, index, buf)
    }

    /// Write CMOS register (CMD_SET_CMOS = 0xE3)
    pub fn set_cmos(&self, index: u16, value: u16) -> Result<(), UsbError> {
        self.control_out(CMD_SET_CMOS, value, index)
    }

    /// Read CMOS register (CMD_GET_CMOS = 0xE4)
    pub fn get_cmos(&self, index: u16, buf: &mut [u8]) -> Result<usize, UsbError> {
        self.control_in(CMD_GET_CMOS, 0, index, buf)
    }



    /// Read internal hardware parameters (LED and Exposure) from EEPROM and 
    /// write them to the CMOS camera and GPIO registers.
    #[allow(dead_code)]
    fn configure_hardware(&self) -> Result<(), UsbError> {
        // 1. Configure LEDs from EEPROM 112 (0x70)
        let mut nvm_buf_led = [0u8; 10];
        for i in 0..10 {
            let mut b = [0u8; 1];
            let _ = self.read_nvm(112 + i, &mut b);
            nvm_buf_led[i as usize] = b[0];
        }

        let main1 = nvm_buf_led[0] as u16;
        let main2 = nvm_buf_led[1] as u16;
        let side1 = nvm_buf_led[2] as u16;
        let side2 = nvm_buf_led[3] as u16;
        let anti1 = nvm_buf_led[4] as u16;
        let anti2 = nvm_buf_led[5] as u16;

        if main1 > 0 { let _ = self.set_gpio(0, (main1 << 8) + 1); }
        if main2 > 0 { let _ = self.set_gpio(1, (main2 << 8) + 1); }
        if side1 > 0 { let _ = self.set_gpio(2, (side1 << 8) + 1); }
        if side2 > 0 { let _ = self.set_gpio(3, (side2 << 8) + 1); }
        if anti1 > 0 { let _ = self.set_gpio(4, (anti1 << 8) + 1); }
        if anti2 > 0 { let _ = self.set_gpio(5, (anti2 << 8) + 1); }

        // 2. Configure Exposure from EEPROM 48 (0x30)
        let mut nvm_buf_exp = [0u8; 12];
        for i in 0..12 {
            let mut b = [0u8; 1];
            let _ = self.read_nvm(48 + i, &mut b);
            nvm_buf_exp[i as usize] = b[0];
        }

        let exposure = u16::from_le_bytes([nvm_buf_exp[0], nvm_buf_exp[1]]);
        let red = u16::from_le_bytes([nvm_buf_exp[2], nvm_buf_exp[3]]);
        let green1 = u16::from_le_bytes([nvm_buf_exp[4], nvm_buf_exp[5]]);
        let green2 = u16::from_le_bytes([nvm_buf_exp[6], nvm_buf_exp[7]]);
        let blue = u16::from_le_bytes([nvm_buf_exp[8], nvm_buf_exp[9]]);

        let write_cam = |addr: i16, val: u8| {
            let _ = self.control_out(0xA5, val as u16, addr as u16);
        };

        if red > 0 {
            write_cam(-91, (red & 0xFF) as u8);
            write_cam(-90, (red & 0xFF) as u8);
        }
        if green1 > 0 {
            write_cam(-93, (green1 & 0xFF) as u8);
            write_cam(-92, (green1 & 0xFF) as u8);
        }
        if green2 > 0 {
            write_cam(-87, (green2 & 0xFF) as u8);
            write_cam(-86, (green2 & 0xFF) as u8);
        }
        if blue > 0 {
            write_cam(-89, (blue & 0xFF) as u8);
            write_cam(-88, (blue & 0xFF) as u8);
        }
        
        // This is actually the Vertical/Horizontal Offset calibration!
        // Writing to registers 3 and 4 centers the fingerprint on the sensor.
        if exposure > 0 {
            write_cam(3, (exposure >> 8) as u8);
            write_cam(4, (exposure & 0xFF) as u8);
        }

        Ok(())
    }

    // --- Device Initialization ---

    /// Initialize the device (full init sequence from SDK)
    ///
    /// 1. Send CMD_INIT (0xE0) control OUT — reset device
    /// 2. Wait 100ms for device stabilization
    /// 3. Drain endpoints and clear bulk halt
    /// 4. Send CMD_HANDSHAKE (0x80) control OUT and read handshake response
    /// 5. Read firmware version via CMD_GET_GPIO with GET_VERSION index
    /// 6. Set hardware image correction modes
    pub fn init(&mut self) -> Result<FirmwareVersion, UsbError> {
        // Step 1: Send CMD_INIT
        self.control_out(CMD_INIT, 0, 0)?;

        // Step 2: Wait for device to stabilize
        std::thread::sleep(INIT_DELAY);

        // Step 3: Drain any stale data from endpoints
        self.drain_buffers();

        // Step 3b: Clear halt on bulk endpoint
        self.handle.clear_halt(BULK_IN_ENDPOINT)?;

        // Step 4: Handshake — send CMD_HANDSHAKE and read bulk response
        self.handshake()?;

        // Step 5: Read firmware version
        let version = self.get_firmware_version()?;

        // Step 6: Configure device for hardware image capture (OUTPUT_MODE = 1)
        let _ = self.set_gpio(GPIO_POWER_MODE, 1);
        let _ = self.set_gpio(GPIO_ANTI_FAKE, 1); // Disable anti-fake check for simpler capture
        let _ = self.set_gpio(GPIO_OUTPUT_MODE, 1);
        self.set_work_mode(MODE_DETECT)?; // 3 = Detect mode
        let _ = self.led_off();

        // Ensure bulk endpoint is ready
        let _ = self.handle.clear_halt(BULK_IN_ENDPOINT);

        self.initialized = true;
        Ok(version)
    }

    /// Perform handshake with the device
    ///
    /// From SDK: `controlEx(0x40, 0x80, 0, 0, buf, 16, 500)` then
    /// `read(endpoint, buf, 16384, 1000)`. If first 4 bytes of bulk
    /// response == 0, handshake is successful.
    pub fn handshake(&self) -> Result<bool, UsbError> {
        // Send handshake command with 16 zero-byte payload
        let handshake_data = [0u8; 16];
        self.handle.write_control(
            CTRL_OUT,
            CMD_HANDSHAKE,
            0,
            0,
            &handshake_data,
            CTRL_TIMEOUT,
        )?;

        // Clear any stale data on the bulk endpoint
        self.handle.clear_halt(BULK_IN_ENDPOINT)?;

        // Read bulk response — use large buffer to avoid Overflow
        // The device may send a large initial block of data
        let mut buf = vec![0u8; 131072]; // 128KB
        let read = match self.handle.read_bulk(
            BULK_IN_ENDPOINT,
            &mut buf,
            Duration::from_secs(5),
        ) {
            Ok(n) => n,
            Err(rusb::Error::Overflow) => {
                // Device sent more data than expected — clear and retry
                self.handle.clear_halt(BULK_IN_ENDPOINT)?;
                // Try reading again with a fresh start
                return Ok(true); // Consider handshake done even with overflow
            }
            Err(e) => return Err(UsbError::Io(e)),
        };

        if read > 4 {
            let code = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            Ok(code == 0)
        } else if read > 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Mark device as initialized (for manual init flow)
    pub fn mark_initialized(&mut self) {
        self.initialized = true;
    }

    // --- Firmware Version ---

    /// Get firmware version via CMD_GET_GPIO with index=0x55
    ///
    /// Returns major.minor version bytes
    pub fn get_firmware_version(&self) -> Result<FirmwareVersion, UsbError> {
        let mut buf = [0u8; 4];
        let read = self.get_gpio(GPIO_GET_VERSION, &mut buf)?;
        if read >= 2 {
            Ok(FirmwareVersion {
                major: buf[0],
                minor: buf[1],
            })
        } else {
            Ok(FirmwareVersion { major: 0, minor: 0 })
        }
    }

    // --- LED Control ---

    /// Control LED on the device via CMD_SET_GPIO with GPIO_LED index
    ///
    /// The ZK9500 LED register is at GPIO index 0x70 (SILKID_LED_ADDR).
    /// Values: 0 = all off, 15 = main (green) on, 16 = fake detect on
    pub fn set_led(&self, color: LedColor, on: bool) -> Result<(), UsbError> {
        let value = if on {
            match color {
                LedColor::Green => LED_MAIN_ON,
                LedColor::Red => LED_FAKE_ON,
                LedColor::White => LED_MAIN_ON, // white maps to main LED
            }
        } else {
            LED_ALL_OFF
        };
        self.set_gpio(GPIO_LED, value)
    }

    /// Turn off all LEDs
    pub fn led_off(&self) -> Result<(), UsbError> {
        self.set_gpio(GPIO_LED, LED_ALL_OFF)
    }

    // --- Work Mode ---

    /// Set work mode (CMD_SET_GPIO with GPIO_WORK_MODE index)
    ///
    /// Modes: IDLE=0, MAIN=1, FAKE=2, DETECT=3
    pub fn set_work_mode(&self, mode: u16) -> Result<(), UsbError> {
        self.set_gpio(GPIO_WORK_MODE, mode)
    }

    // --- Image Capture ---

    /// Capture a fingerprint image (blocking until finger detected or timeout)
    ///
    /// Dynamically adapts to both 288x375 (108,000 bytes) and 300x375 (112,500/114,688 bytes)
    /// hardware revisions of the ZK9500 optical sensor.
    pub fn capture_image(&self) -> Result<RawImage, UsbError> {
        if !self.initialized {
            return Err(UsbError::NotInitialized);
        }

        let mut timeout_count = 0;
        let max_timeouts = MAX_CAPTURE_ATTEMPTS;

        // Always clear image buffer before capturing a new frame
        let _ = self.clear_image();
        
        loop {
            // Check if finger is present and image is ready
            let mut status_buf = [0u8; 10];
            let read_len = match self.control_in(CMD_DET_IMAGE, 0, 0, &mut status_buf) {
                Ok(len) => len,
                Err(_) => {
                    // Control transfer failed, retry
                    std::thread::sleep(Duration::from_millis(50));
                    continue;
                }
            };

            if read_len >= 1 && status_buf[0] == 1 {
                // Image is ready, read full stream from bulk endpoint
                let mut raw_bytes = Vec::with_capacity(131072);
                let mut chunk_buf = vec![0u8; 16384];
                let start_read = std::time::Instant::now();
                
                while start_read.elapsed() < Duration::from_secs(3) {
                    if raw_bytes.len() >= 112500 {
                        break;
                    }

                    match self.handle.read_bulk(
                        BULK_IN_ENDPOINT,
                        &mut chunk_buf,
                        Duration::from_millis(400),
                    ) {
                        Ok(n) if n > 0 => {
                            raw_bytes.extend_from_slice(&chunk_buf[..n]);
                            // Short packet indicates end of USB bulk transfer
                            if n < 16384 && raw_bytes.len() >= 108000 {
                                break;
                            }
                            if raw_bytes.len() >= 114688 {
                                break;
                            }
                        }
                        Ok(_) => {}
                        Err(rusb::Error::Timeout) => {
                            if !raw_bytes.is_empty() {
                                break; // Bulk transmission completed
                            }
                        }
                        Err(rusb::Error::Overflow) | Err(rusb::Error::Pipe) => {
                            let _ = self.handle.clear_halt(BULK_IN_ENDPOINT);
                            if raw_bytes.len() >= 100000 {
                                break; // End of transmission
                            }
                        }
                        Err(rusb::Error::Io) => {
                            let _ = self.handle.clear_halt(BULK_IN_ENDPOINT);
                            if raw_bytes.len() >= 100000 {
                                break; // End of transmission on packet boundary
                            }
                            if raw_bytes.is_empty() {
                                std::thread::sleep(Duration::from_millis(50));
                            }
                        }
                        Err(e) => {
                            if raw_bytes.len() >= 100000 {
                                let _ = self.handle.clear_halt(BULK_IN_ENDPOINT);
                                break;
                            }
                            return Err(UsbError::Io(e));
                        }
                    }
                }
                
                let _ = self.handle.clear_halt(BULK_IN_ENDPOINT);
                
                let total_len = raw_bytes.len();
                if total_len >= 112500 {
                    // 300x375 sensor revision (112,500 bytes)
                    raw_bytes.truncate(112500);
                    return RawImage::new(300, 375, DEFAULT_DPI, raw_bytes);
                } else if total_len >= 108000 {
                    // 288x375 sensor revision (108,000 bytes)
                    raw_bytes.truncate(108000);
                    return RawImage::new(288, 375, DEFAULT_DPI, raw_bytes);
                } else if total_len > 0 {
                    let width = 300;
                    let height = (total_len / width as usize) as u32;
                    if height >= 100 {
                        let valid_len = (width * height) as usize;
                        raw_bytes.truncate(valid_len);
                        return RawImage::new(width, height, DEFAULT_DPI, raw_bytes);
                    }
                }

                // Didn't read enough, clear image and retry
                let _ = self.clear_image();
                timeout_count += 1;
                if timeout_count >= max_timeouts {
                    return Err(UsbError::CaptureTimeout((max_timeouts * 100) as u64));
                }
                continue; 
            } else {
                // Finger not present or image not ready yet
                timeout_count += 1;
                if timeout_count >= max_timeouts {
                    return Err(UsbError::CaptureTimeout((max_timeouts * 100) as u64));
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }

    /// Detect if a finger is present on the scanner using hardware status
    ///
    /// Uses CMD_DET_IMAGE (0xEA) - returns true if finger is detected by the hardware
    pub fn is_finger_present(&self) -> Result<bool, UsbError> {
        let mut buf = [0u8; 10];
        match self.control_in(CMD_DET_IMAGE, 0, 0, &mut buf) {
            Ok(len) if len >= 1 => Ok(buf[0] == 1),
            Ok(_) => Ok(false),
            Err(_) => Ok(false),
        }
    }

    /// Block until the finger is removed from the scanner (hardware-based)
    ///
    /// Polls CMD_DET_IMAGE until status_buf[0] == 0
    pub fn wait_finger_removed(&self, timeout_secs: u64) -> Result<(), UsbError> {
        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);
        loop {
            if start.elapsed() > timeout {
                return Ok(()); // Timeout — assume removed and continue
            }
            if !self.is_finger_present()? {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(150));
        }
    }

    /// Beep the device buzzer
    ///
    /// The ZK9500 doesn't have a dedicated beep command in the SDK protocol.
    /// Buzzer control is typically done via CMD_SET_GPIO with a specific register.
    /// This is a placeholder — exact register needs verification on real device.
    pub fn beep(&self, _duration_ms: u16) -> Result<(), UsbError> {
        // TODO: Verify buzzer GPIO register on real device
        // Possible: CMD_SET_GPIO with a buzger register index
        Ok(())
    }

    /// Clear image buffer (CMD_CLR_IMAGE = 0xEB)
    pub fn clear_image(&self) -> Result<(), UsbError> {
        self.control_out(CMD_CLR_IMAGE, 0, 0)
    }

    // --- NVM Access ---

    /// Write to non-volatile memory (CMD_SET_NVM = 0xE6)
    pub fn write_nvm(&self, index: u16, data: &[u8]) -> Result<(), UsbError> {
        self.handle.write_control(
            CTRL_OUT,
            CMD_SET_NVM,
            data.len() as u16,
            index,
            data,
            CTRL_TIMEOUT,
        )?;
        Ok(())
    }

    /// Read from non-volatile memory (CMD_GET_NVM = 0xE7)
    pub fn read_nvm(&self, index: u16, buf: &mut [u8]) -> Result<(), UsbError> {
        self.handle.read_control(
            CTRL_IN,
            CMD_GET_NVM,
            buf.len() as u16,
            index,
            buf,
            CTRL_TIMEOUT,
        )?;
        Ok(())
    }

    // --- Device Info ---

    /// Get device serial number from USB descriptor
    pub fn get_serial(&self) -> Result<String, UsbError> {
        let desc = self.handle.device().device_descriptor()?;
        let serial = self.handle.read_serial_number_string_ascii(&desc)
            .map_err(UsbError::Io)?;
        Ok(serial)
    }

    /// Get device info from USB descriptor
    pub fn device_info(&self) -> Result<DeviceInfo, UsbError> {
        let desc = self.handle.device().device_descriptor()?;
        let manufacturer = self.handle.read_manufacturer_string_ascii(&desc).unwrap_or_default();
        let product = self.handle.read_product_string_ascii(&desc).unwrap_or_default();
        let serial = self.handle.read_serial_number_string_ascii(&desc).ok();

        Ok(DeviceInfo {
            vid: desc.vendor_id(),
            pid: desc.product_id(),
            manufacturer,
            product,
            serial,
            bus_number: self.handle.device().bus_number(),
            address: self.handle.device().address(),
        })
    }

    /// Close the device and release USB interface
    pub fn close(self) -> Result<(), UsbError> {
        self.handle.release_interface(USB_INTERFACE)?;
        Ok(())
    }
}

impl Drop for Zk9500 {
    fn drop(&mut self) {
        if self.initialized {
            let _ = self.set_work_mode(MODE_IDLE);
            let _ = self.led_off();
        }
        let _ = self.handle.release_interface(USB_INTERFACE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(ZKTECO_VID, 0x1B55);
        assert_eq!(ZK9500_PID, 0x0124);
        assert_eq!(IMAGE_SIZE, 112500);
        assert_eq!(CMD_INIT, 0xE0);
        assert_eq!(CMD_SET_GPIO, 0xE1);
        assert_eq!(CMD_GET_GPIO, 0xE2);
        assert_eq!(CMD_GET_IMAGE, 0xE5);
        assert_eq!(CTRL_OUT, 0x40);
        assert_eq!(CTRL_IN, 0xC0);
        assert_eq!(GPIO_GET_VERSION, 0x55);
        assert_eq!(GPIO_LED, 0x70);
        assert_eq!(BULK_IN_ENDPOINT, 0x82);
        assert_eq!(INTERRUPT_IN_ENDPOINT, 0x81);
    }

    #[test]
    fn test_raw_image_create() {
        let data = vec![128u8; IMAGE_SIZE];
        let img = RawImage::default_size(data).unwrap();
        assert_eq!(img.width, 300);
        assert_eq!(img.height, 375);
        assert_eq!(img.dpi, 500);
        assert_eq!(img.pixel(0, 0), 128);
    }

    #[test]
    fn test_raw_image_invalid_size() {
        let data = vec![0u8; 100];
        let result = RawImage::default_size(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_firmware_version_display() {
        let v = FirmwareVersion { major: 2, minor: 5 };
        assert_eq!(format!("{}", v), "2.5");
    }

    #[test]
    fn test_is_device_present() {
        // Should not panic regardless of USB availability
        let _ = is_device_present();
    }
}
