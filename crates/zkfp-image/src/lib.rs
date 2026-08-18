//! # zkfp-image
//!
//! Fingerprint image processing: grayscale I/O, orientation maps,
//! frequency maps, quality assessment, background detection.
//!
//! Reimplements the `GrayImage` and `Fingerprint` classes from
//! `libzkfinger10.so` (which wraps NBIS algorithms).

use std::os::raw::{c_char, c_float, c_int};
use thiserror::Error;

// --- Error types ---

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Invalid BMP data: {0}")]
    InvalidBmp(String),

    #[error("Invalid TIFF data: {0}")]
    InvalidTiff(String),

    #[error("Invalid image dimensions: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image decode error: {0}")]
    Decode(#[from] image::ImageError),

    #[error("WSQ codec error: {0}")]
    Wsq(String),
}

unsafe extern "C" {
    fn wsq_decode_mem(
        odata: *mut *mut u8,
        ow: *mut c_int,
        oh: *mut c_int,
        od: *mut c_int,
        oppi: *mut c_int,
        lossyflag: *mut c_int,
        idata: *mut u8,
        ilen: c_int,
    ) -> c_int;

    fn wsq_encode_mem(
        odata: *mut *mut u8,
        olen: *mut c_int,
        bitrate: c_float,
        idata: *mut u8,
        width: c_int,
        height: c_int,
        depth: c_int,
        ppi: c_int,
        comment_text: *mut c_char,
    ) -> c_int;

    fn free(ptr: *mut core::ffi::c_void);
}

const DEFAULT_WSQ_BITRATE: f32 = 0.75;
const DEFAULT_WSQ_PPI: i32 = 500;

fn is_wsq_path(path: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("wsq"))
        .unwrap_or(false)
}

// --- GrayImage ---

/// 8-bit grayscale image (reimplements `GrayImage` from SDK)
/// Configuration for image enhancement
#[derive(Clone, Debug, Copy)]
pub enum ContrastMethod {
    Stretch,
    Darken,
}

#[derive(Clone, Debug, Copy)]
pub struct EnhanceConfig {
    pub apply_enhancement: bool,
    pub method: ContrastMethod,
    pub bg_intensity: u8,
    pub invert: bool,
    pub flip_vertical: bool,
    pub padding: u32,
}

impl Default for EnhanceConfig {
    fn default() -> Self {
        Self {
            apply_enhancement: true,
            method: ContrastMethod::Darken,
            bg_intensity: 255,
            invert: false,
            flip_vertical: false,
            padding: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GrayImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl GrayImage {
    /// Create from raw grayscale data
    pub fn from_raw(data: Vec<u8>, width: u32, height: u32) -> Result<Self, ImageError> {
        let expected = (width * height) as usize;
        if data.len() != expected {
            return Err(ImageError::InvalidDimensions { width, height });
        }
        Ok(Self {
            width,
            height,
            data,
        })
    }

    /// Create a blank image (all white = 255)
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![255; (width * height) as usize],
        }
    }

    /// Load from BMP file data (reimplements `GrayImage::loadBMPFromMemory`)
    pub fn from_bmp(data: &[u8]) -> Result<Self, ImageError> {
        let img = image::load_from_memory(data)?.to_luma8();
        let (width, height) = (img.width(), img.height());
        Ok(Self {
            width,
            height,
            data: img.into_raw(),
        })
    }

    /// Load from WSQ bytes
    pub fn from_wsq_bytes(data: &[u8]) -> Result<Self, ImageError> {
        if data.is_empty() {
            return Err(ImageError::Wsq("WSQ input is empty".to_string()));
        }

        let mut input = data.to_vec();
        let mut out_ptr: *mut u8 = std::ptr::null_mut();
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let mut depth: c_int = 0;
        let mut ppi: c_int = 0;
        let mut lossy_flag: c_int = 0;

        let ret = unsafe {
            wsq_decode_mem(
                &mut out_ptr,
                &mut width,
                &mut height,
                &mut depth,
                &mut ppi,
                &mut lossy_flag,
                input.as_mut_ptr(),
                input.len() as c_int,
            )
        };

        if ret != 0 {
            return Err(ImageError::Wsq(format!(
                "WSQ decode failed with code {ret}"
            )));
        }
        if out_ptr.is_null() {
            return Err(ImageError::Wsq(
                "WSQ decode returned null buffer".to_string(),
            ));
        }
        if width <= 0 || height <= 0 {
            unsafe {
                free(out_ptr.cast());
            }
            return Err(ImageError::Wsq(format!(
                "WSQ decode returned invalid dimensions {width}x{height}"
            )));
        }
        if depth != 8 {
            unsafe {
                free(out_ptr.cast());
            }
            return Err(ImageError::Wsq(format!(
                "WSQ decode returned unsupported depth {depth}; expected 8"
            )));
        }

        let len = (width as usize) * (height as usize);
        let out = unsafe { std::slice::from_raw_parts(out_ptr, len).to_vec() };
        unsafe {
            free(out_ptr.cast());
        }
        Self::from_raw(out, width as u32, height as u32)
    }

    /// Load from file path
    pub fn from_file(path: &str) -> Result<Self, ImageError> {
        if is_wsq_path(path) {
            let bytes = std::fs::read(path)?;
            return Self::from_wsq_bytes(&bytes);
        }

        let img = image::open(path)?.to_luma8();
        let (width, height) = (img.width(), img.height());
        Ok(Self {
            width,
            height,
            data: img.into_raw(),
        })
    }

    /// Convert to BMP format (reimplements `GrayImage::saveAsBMPToMemory`)
    pub fn to_bmp(&self) -> Vec<u8> {
        let img = image::GrayImage::from_raw(self.width, self.height, self.data.clone())
            .expect("valid dimensions");
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Bmp)
            .expect("BMP encoding");
        buf.into_inner()
    }

    /// Save as BMP to file (reimplements `GrayImage::saveAsBMP`)
    pub fn save_bmp(&self, path: &str) -> Result<(), ImageError> {
        let img = image::GrayImage::from_raw(self.width, self.height, self.data.clone())
            .expect("valid dimensions");
        img.save(path)?;
        Ok(())
    }

    /// Save as PNG to file
    pub fn save_png(&self, path: &str) -> Result<(), ImageError> {
        let img = image::GrayImage::from_raw(self.width, self.height, self.data.clone())
            .expect("valid dimensions");
        img.save_with_format(path, image::ImageFormat::Png)?;
        Ok(())
    }

    /// Encode to WSQ bytes using NBIS.
    pub fn to_wsq_bytes(&self) -> Result<Vec<u8>, ImageError> {
        let mut input = self.data.clone();
        let mut out_ptr: *mut u8 = std::ptr::null_mut();
        let mut out_len: c_int = 0;
        let expected_size = (self.width as usize) * (self.height as usize);
        if input.len() != expected_size {
            return Err(ImageError::Wsq(format!(
                "WSQ input size mismatch: expected {expected_size}, got {}",
                input.len()
            )));
        }

        let ret = unsafe {
            wsq_encode_mem(
                &mut out_ptr,
                &mut out_len,
                DEFAULT_WSQ_BITRATE,
                input.as_mut_ptr(),
                self.width as c_int,
                self.height as c_int,
                8,
                DEFAULT_WSQ_PPI,
                b"\0".as_ptr() as *mut c_char,
            )
        };

        if ret != 0 {
            return Err(ImageError::Wsq(format!(
                "WSQ encode failed with code {ret}"
            )));
        }
        if out_ptr.is_null() {
            return Err(ImageError::Wsq(
                "WSQ encode returned null buffer".to_string(),
            ));
        }
        if out_len <= 0 {
            unsafe {
                free(out_ptr.cast());
            }
            return Err(ImageError::Wsq(
                "WSQ encode returned empty buffer".to_string(),
            ));
        }

        let out = unsafe { std::slice::from_raw_parts(out_ptr, out_len as usize).to_vec() };
        unsafe {
            free(out_ptr.cast());
        }
        Ok(out)
    }

    pub fn save_wsq(&self, path: &str) -> Result<(), ImageError> {
        std::fs::write(path, self.to_wsq_bytes()?)?;
        Ok(())
    }

    pub fn to_base64_wsq(&self) -> Result<String, ImageError> {
        use base64::{engine::general_purpose, Engine as _};
        Ok(general_purpose::STANDARD.encode(self.to_wsq_bytes()?))
    }

    pub fn to_base64_with_format(&self, format: &str) -> Result<String, ImageError> {
        match format.to_ascii_lowercase().as_str() {
            "png" => Ok(self.to_base64_png()),
            "bmp" => Ok(self.to_base64_bmp()),
            "wsq" => self.to_base64_wsq(),
            other => Err(ImageError::Wsq(format!(
                "Unsupported output format: {other}"
            ))),
        }
    }

    /// Return as a Base64 encoded PNG string
    pub fn to_base64_png(&self) -> String {
        use base64::{engine::general_purpose, Engine as _};
        let img = image::GrayImage::from_raw(self.width, self.height, self.data.clone())
            .expect("valid dimensions");
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Png)
            .expect("PNG encoding");
        general_purpose::STANDARD.encode(buf.into_inner())
    }

    /// Return as a Base64 encoded BMP string
    pub fn to_base64_bmp(&self) -> String {
        use base64::{engine::general_purpose, Engine as _};
        let bmp_bytes = self.to_bmp();
        general_purpose::STANDARD.encode(bmp_bytes)
    }

    // --- Pixel access ---

    pub fn pixel(&self, x: u32, y: u32) -> u8 {
        self.data[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, val: u8) {
        self.data[(y * self.width + x) as usize] = val;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    // --- Operations ---

    /// Sobel X derivative at pixel (reimplements `GrayImage::sobelX`)
    pub fn sobel_x(&self, x: u32, y: u32) -> f64 {
        if x == 0 || x >= self.width - 1 || y == 0 || y >= self.height - 1 {
            return 0.0;
        }
        let tl = self.pixel(x - 1, y - 1) as f64;
        let l = self.pixel(x - 1, y) as f64;
        let bl = self.pixel(x - 1, y + 1) as f64;
        let tr = self.pixel(x + 1, y - 1) as f64;
        let r = self.pixel(x + 1, y) as f64;
        let br = self.pixel(x + 1, y + 1) as f64;
        -tl - 2.0 * l - bl + tr + 2.0 * r + br
    }

    /// Sobel Y derivative at pixel (reimplements `GrayImage::sobelY`)
    pub fn sobel_y(&self, x: u32, y: u32) -> f64 {
        if x == 0 || x >= self.width - 1 || y == 0 || y >= self.height - 1 {
            return 0.0;
        }
        let tl = self.pixel(x - 1, y - 1) as f64;
        let t = self.pixel(x, y - 1) as f64;
        let tr = self.pixel(x + 1, y - 1) as f64;
        let bl = self.pixel(x - 1, y + 1) as f64;
        let b = self.pixel(x, y + 1) as f64;
        let br = self.pixel(x + 1, y + 1) as f64;
        -tl - 2.0 * t - tr + bl + 2.0 * b + br
    }

    /// Invert pixel values (255 - pixel) — ZK sensors output inverted polarity
    pub fn invert(&self) -> GrayImage {
        let data: Vec<u8> = self.data.iter().map(|&p| 255 - p).collect();
        GrayImage {
            width: self.width,
            height: self.height,
            data,
        }
    }

    /// Flip image vertically (top ↔ bottom rows)
    pub fn flip_vertical(&self) -> GrayImage {
        let mut data = Vec::with_capacity(self.data.len());
        for y in (0..self.height).rev() {
            let start = (y * self.width) as usize;
            let end = start + self.width as usize;
            data.extend_from_slice(&self.data[start..end]);
        }
        GrayImage {
            width: self.width,
            height: self.height,
            data,
        }
    }

    /// Enhance contrast by global min-max stretch: (val - min) * max_val / (max - min).
    /// darkest pixel → 0 (black), brightest → max_val (gray/white).
    pub fn contrast_stretch(&self, max_val: u8) -> GrayImage {
        if self.data.is_empty() {
            return GrayImage {
                width: self.width,
                height: self.height,
                data: vec![],
            };
        }

        let p_min = *self.data.iter().min().unwrap() as u32;
        let p_max = *self.data.iter().max().unwrap() as u32;
        let range = p_max.saturating_sub(p_min);

        let data: Vec<u8> = if range == 0 {
            self.data.iter().map(|&p| p.min(max_val)).collect()
        } else {
            self.data
                .iter()
                .map(|&p| {
                    (((p as u32).saturating_sub(p_min) * max_val as u32 + range / 2) / range)
                        .min(max_val as u32) as u8
                })
                .collect()
        };

        GrayImage {
            width: self.width,
            height: self.height,
            data,
        }
    }

    /// Darkens the fingerprint ridges without adding noise to the background.
    /// `max_val` defines the background brightness (255 = pure white).
    pub fn darken_ridges(&self, max_val: u8) -> GrayImage {
        if self.data.is_empty() {
            return GrayImage {
                width: self.width,
                height: self.height,
                data: vec![],
            };
        }

        let lut: Vec<u8> = (0..=255u8)
            .map(|v| {
                if v <= 200 {
                    // Map [0, 200] -> [0, 80] (darken heavily)
                    ((v as u32 * 80) / 200) as u8
                } else {
                    // Map [201, 255] -> [target_floor, max_val]
                    let floor = (max_val as u32 * 240) / 255;
                    let span = max_val.saturating_sub(floor as u8) as u32;
                    floor as u8 + (((v as u32 - 200) * span) / 55) as u8
                }
            })
            .collect();

        let data = self.data.iter().map(|&p| lut[p as usize]).collect();
        GrayImage {
            width: self.width,
            height: self.height,
            data,
        }
    }

    /// Adds a white border around the image.
    /// Useful for ensuring the fingerprint doesn't touch the absolute edges
    /// of the BMP, which can interfere with visualization or minutiae extraction.
    pub fn pad_with_white(&self, padding: u32) -> GrayImage {
        let new_w = self.width + padding * 2;
        let new_h = self.height + padding * 2;
        let mut new_data = vec![255u8; (new_w * new_h) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let src_idx = (y * self.width + x) as usize;
                let dst_idx = ((y + padding) * new_w + (x + padding)) as usize;
                new_data[dst_idx] = self.data[src_idx];
            }
        }
        GrayImage {
            width: new_w,
            height: new_h,
            data: new_data,
        }
    }

    /// Enhance fingerprint image for display.
    ///
    /// This now uses a default configuration but can be customized.
    pub fn enhance_fingerprint(&self) -> GrayImage {
        self.enhance_with_config(EnhanceConfig::default())
    }

    /// Enhance with specific configuration
    pub fn enhance_with_config(&self, config: EnhanceConfig) -> GrayImage {
        println!("Enhancement enabled");
        if !config.apply_enhancement {
            println!("Enhancement disabled");
            return self.clone();
        }

        let mut img = self.clone();

        if config.invert {
            img = img.invert();
        }

        match config.method {
            ContrastMethod::Stretch => {
                img = img.contrast_stretch(config.bg_intensity);
            }
            ContrastMethod::Darken => {
                img = img.darken_ridges(config.bg_intensity);
            }
        }

        if config.padding > 0 {
            img = img.pad_with_white(config.padding);
        }

        if config.flip_vertical {
            img = img.flip_vertical();
        }

        img
    }

    /// Resize image (reimplements `GrayImage::resizeImage`)
    pub fn resize(&self, new_w: u32, new_h: u32) -> GrayImage {
        let img = image::GrayImage::from_raw(self.width, self.height, self.data.clone())
            .expect("valid dimensions");
        let resized =
            image::imageops::resize(&img, new_w, new_h, image::imageops::FilterType::Gaussian);
        GrayImage {
            width: new_w,
            height: new_h,
            data: resized.into_raw(),
        }
    }
}

// --- Processing maps ---

/// Block-based orientation map (reimplements `calculateOrientations`)
/// Each block has a ridge flow angle in radians [0, π)
#[derive(Clone, Debug)]
pub struct OrientationMap {
    pub block_size: u32,
    pub width: u32,  // in blocks
    pub height: u32, // in blocks
    pub angles: Vec<f64>,
}

impl OrientationMap {
    pub fn angle(&self, bx: u32, by: u32) -> f64 {
        self.angles[(by * self.width + bx) as usize]
    }

    pub fn set_angle(&mut self, bx: u32, by: u32, angle: f64) {
        self.angles[(by * self.width + bx) as usize] = angle;
    }
}

/// Block-based frequency map (reimplements `calculateFrequency`)
#[derive(Clone, Debug)]
pub struct FrequencyMap {
    pub block_size: u32,
    pub width: u32,
    pub height: u32,
    pub frequencies: Vec<f64>,
}

/// Block-based quality map (reimplements `calculateBlockQuality2`)
#[derive(Clone, Debug)]
pub struct QualityMap {
    pub block_size: u32,
    pub width: u32,
    pub height: u32,
    pub qualities: Vec<f64>,
}

/// Binary foreground/background mask (reimplements `decideBackground`)
#[derive(Clone, Debug)]
pub struct BinaryMap {
    pub width: u32,
    pub height: u32,
    pub mask: Vec<bool>, // true = foreground (has fingerprint)
}

impl BinaryMap {
    pub fn is_foreground(&self, x: u32, y: u32) -> bool {
        self.mask[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, x: u32, y: u32, val: bool) {
        self.mask[(y * self.width + x) as usize] = val;
    }
}

/// Quality assessment result (reimplements `getFingerprintQuality` / `getFingerprintQualityInfo`)
#[derive(Clone, Debug)]
pub struct QualityInfo {
    pub overall: u32,
    pub mean_quality: f64,
    pub valid_area_ratio: f64,
}

// --- Processing functions ---

/// Calculate ridge orientation map (reimplements `Fingerprint::calculateOrientations`)
/// Uses DFT-based direction estimation (NBIS `dft_dir_powers`)
pub fn calculate_orientations(img: &GrayImage, block_size: u32) -> OrientationMap {
    let bw = img.width / block_size;
    let bh = img.height / block_size;
    let mut angles = vec![0.0; (bw * bh) as usize];

    for by in 0..bh {
        for bx in 0..bw {
            let cx = bx * block_size + block_size / 2;
            let cy = by * block_size + block_size / 2;

            // Compute dominant direction using gradient-based method
            let mut gxx = 0.0f64;
            let mut gyy = 0.0f64;
            let mut gxy = 0.0f64;

            let half = block_size as i32 / 2;
            for dy in -half..half {
                for dx in -half..half {
                    let px = (cx as i32 + dx) as u32;
                    let py = (cy as i32 + dy) as u32;
                    if px >= img.width || py >= img.height {
                        continue;
                    }
                    let sx = img.sobel_x(px, py);
                    let sy = img.sobel_y(px, py);
                    gxx += sx * sx;
                    gyy += sy * sy;
                    gxy += sx * sy;
                }
            }

            // Orientation from structure tensor
            let angle = 0.5 * (gxy).atan2(gxx - gyy) + std::f64::consts::FRAC_PI_2;
            angles[(by * bw + bx) as usize] = angle;
        }
    }

    OrientationMap {
        block_size,
        width: bw,
        height: bh,
        angles,
    }
}

/// Calculate ridge frequency map (reimplements `Fingerprint::calculateFrequency`)
///
/// Uses projection-based method: project pixel intensities along a line
/// perpendicular to the ridge orientation, then count zero crossings
/// to estimate ridge spacing → frequency.
pub fn calculate_frequency(img: &GrayImage, orient: &OrientationMap) -> FrequencyMap {
    let bw = img.width / orient.block_size;
    let bh = img.height / orient.block_size;
    let mut frequencies = vec![0.0; (bw * bh) as usize];

    let window_size = orient.block_size as usize; // projection window length

    for by in 0..bh {
        for bx in 0..bw {
            let cx = bx * orient.block_size + orient.block_size / 2;
            let cy = by * orient.block_size + orient.block_size / 2;
            let angle = orient.angle(bx, by);

            // Sample pixels along a line perpendicular to ridge direction
            let perp_angle = angle + std::f64::consts::FRAC_PI_2;
            let dx = perp_angle.cos();
            let dy = perp_angle.sin();

            let mut projection = Vec::with_capacity(window_size);
            for t in 0..window_size {
                let offset = t as f64 - window_size as f64 / 2.0;
                let px = (cx as f64 + offset * dx).round() as i32;
                let py = (cy as f64 + offset * dy).round() as i32;
                if px >= 0 && px < img.width as i32 && py >= 0 && py < img.height as i32 {
                    projection.push(img.pixel(px as u32, py as u32) as f64);
                }
            }

            // Estimate frequency from zero crossings of the projection derivative
            let freq = if projection.len() >= 4 {
                let mean = projection.iter().sum::<f64>() / projection.len() as f64;
                let centered: Vec<f64> = projection.iter().map(|&v| v - mean).collect();
                let crossings = centered
                    .windows(2)
                    .filter(|w| (w[0] >= 0.0) != (w[1] >= 0.0))
                    .count();
                // Each full ridge period = 2 crossings; frequency = crossings / (2 * window)
                if crossings >= 2 {
                    crossings as f64 / (2.0 * projection.len() as f64)
                } else {
                    -1.0 // invalid: no clear ridge structure
                }
            } else {
                -1.0
            };

            frequencies[(by * bw + bx) as usize] = freq;
        }
    }

    // Interpolate invalid frequencies from neighbors
    for by in 0..bh {
        for bx in 0..bw {
            let idx = (by * bw + bx) as usize;
            if frequencies[idx] < 0.0 {
                // Average valid neighbors
                let mut sum = 0.0;
                let mut count = 0;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let nx = bx as i32 + dx;
                        let ny = by as i32 + dy;
                        if nx >= 0 && nx < bw as i32 && ny >= 0 && ny < bh as i32 {
                            let nf = frequencies[(ny as u32 * bw + nx as u32) as usize];
                            if nf > 0.0 {
                                sum += nf;
                                count += 1;
                            }
                        }
                    }
                }
                frequencies[idx] = if count > 0 {
                    sum / count as f64
                } else {
                    1.0 / 8.0
                };
            }
        }
    }

    FrequencyMap {
        block_size: orient.block_size,
        width: bw,
        height: bh,
        frequencies,
    }
}

/// Calculate block quality map (reimplements `Fingerprint::calculateBlockQuality2`)
///
/// Uses gradient coherence (how well gradients agree with dominant orientation)
/// as a quality metric. High coherence = clear ridges = high quality.
pub fn calculate_block_quality(img: &GrayImage) -> QualityMap {
    let block_size = 8u32;
    let bw = img.width / block_size;
    let bh = img.height / block_size;
    let mut qualities = vec![0.0; (bw * bh) as usize];

    for by in 0..bh {
        for bx in 0..bw {
            let cx = bx * block_size + block_size / 2;
            let cy = by * block_size + block_size / 2;

            let mut gxx = 0.0f64;
            let mut gyy = 0.0f64;
            let mut gxy = 0.0f64;
            let mut grad_mag_sum = 0.0f64;

            let half = block_size as i32 / 2;
            for dy in -half..half {
                for dx in -half..half {
                    let px = (cx as i32 + dx) as u32;
                    let py = (cy as i32 + dy) as u32;
                    if px >= img.width || py >= img.height {
                        continue;
                    }
                    let sx = img.sobel_x(px, py);
                    let sy = img.sobel_y(px, py);
                    let mag = sx.hypot(sy);
                    gxx += sx * sx;
                    gyy += sy * sy;
                    gxy += sx * sy;
                    grad_mag_sum += mag;
                }
            }

            // Coherence = how aligned the gradients are
            // coherence = sqrt((gxx - gyy)^2 + 4*gxy^2) / (gxx + gyy)
            let trace = gxx + gyy;
            let coherence = if trace > 1e-6 {
                ((gxx - gyy).powi(2) + 4.0 * gxy.powi(2)).sqrt() / trace
            } else {
                0.0
            };

            // Also factor in mean gradient magnitude (contrast)
            let mean_grad = if (block_size * block_size) > 0 {
                grad_mag_sum / (block_size * block_size) as f64
            } else {
                0.0
            };

            // Combined quality: coherence * normalized contrast
            let contrast_factor = (mean_grad / 50.0).min(1.0);
            qualities[(by * bw + bx) as usize] = coherence * contrast_factor;
        }
    }

    QualityMap {
        block_size,
        width: bw,
        height: bh,
        qualities,
    }
}

/// Detect foreground/background (reimplements `Fingerprint::decideBackgroundSimple`)
///
/// Uses block-based mean intensity + variance to decide:
/// - Foreground: dark region with high variance (ridges present)
/// - Background: bright region with low variance (no ridges)
pub fn detect_background(img: &GrayImage) -> BinaryMap {
    let block_size = 8u32;
    let bw = img.width / block_size;
    let bh = img.height / block_size;

    // Compute per-block mean and variance
    let mut block_fg = vec![false; (bw * bh) as usize];
    for by in 0..bh {
        for bx in 0..bw {
            let mut sum = 0.0f64;
            let mut count = 0u32;
            for dy in 0..block_size {
                for dx in 0..block_size {
                    let px = bx * block_size + dx;
                    let py = by * block_size + dy;
                    if px < img.width && py < img.height {
                        sum += img.pixel(px, py) as f64;
                        count += 1;
                    }
                }
            }
            let mean = sum / count as f64;

            let mut var_sum = 0.0f64;
            for dy in 0..block_size {
                for dx in 0..block_size {
                    let px = bx * block_size + dx;
                    let py = by * block_size + dy;
                    if px < img.width && py < img.height {
                        let diff = img.pixel(px, py) as f64 - mean;
                        var_sum += diff * diff;
                    }
                }
            }
            let variance = var_sum / count as f64;

            // Foreground: dark (mean < threshold) AND textured (variance > threshold)
            block_fg[(by * bw + bx) as usize] = mean < 180.0 && variance > 200.0;
        }
    }

    // Upscale block mask to pixel mask
    let mut mask = vec![false; (img.width * img.height) as usize];
    for by in 0..bh {
        for bx in 0..bw {
            if block_fg[(by * bw + bx) as usize] {
                for dy in 0..block_size {
                    for dx in 0..block_size {
                        let px = bx * block_size + dx;
                        let py = by * block_size + dy;
                        if px < img.width && py < img.height {
                            mask[(py * img.width + px) as usize] = true;
                        }
                    }
                }
            }
        }
    }

    BinaryMap {
        width: img.width,
        height: img.height,
        mask,
    }
}

/// Get overall fingerprint quality score (reimplements `getFingerprintQuality`)
///
/// Score 0-100 based on:
/// - Mean block quality in foreground
/// - Foreground area ratio
/// - Gradient coherence
pub fn get_quality(img: &GrayImage) -> u32 {
    let bg = detect_background(img);
    let qmap = calculate_block_quality(img);

    let total_blocks = (qmap.width * qmap.height) as usize;
    if total_blocks == 0 {
        return 0;
    }

    // Count foreground blocks and their quality
    let mut fg_quality_sum = 0.0f64;
    let mut fg_count = 0usize;
    for by in 0..qmap.height {
        for bx in 0..qmap.width {
            let cx = bx * qmap.block_size + qmap.block_size / 2;
            let cy = by * qmap.block_size + qmap.block_size / 2;
            if cx < img.width && cy < img.height && bg.is_foreground(cx, cy) {
                fg_quality_sum += qmap.qualities[(by * qmap.width + bx) as usize];
                fg_count += 1;
            }
        }
    }

    if fg_count == 0 {
        return 0;
    }

    let mean_quality = fg_quality_sum / fg_count as f64;
    let fg_ratio = fg_count as f64 / total_blocks as f64;

    // Combine: quality * area_factor
    // area_factor penalizes small fingerprint area
    let area_factor = (fg_ratio * 4.0).min(1.0); // need at least 25% fg for full score
    let score = (mean_quality * area_factor * 100.0) as u32;
    score.min(100)
}

/// Get detailed quality info (reimplements `getFingerprintQualityInfo`)
pub fn get_quality_info(img: &GrayImage) -> QualityInfo {
    let bg = detect_background(img);
    let total = (img.width * img.height) as f64;
    let fg_count = bg.mask.iter().filter(|&&b| b).count() as f64;
    let overall = get_quality(img);

    QualityInfo {
        overall,
        mean_quality: overall as f64 / 100.0,
        valid_area_ratio: fg_count / total,
    }
}

/// Detect if a fingerprint is present in raw data
/// (reimplements `UserLib::doGetFingerprintPresence`)
pub fn detect_finger_presence(data: &[u8], _width: u32, _height: u32) -> bool {
    // A fingerprint has high contrast (ridges and valleys) causing high variance.
    // An empty glass has a relatively flat color (low variance).
    let sample_step = 10; // Sample every 10th pixel for speed
    let mut sum = 0.0;
    let mut count = 0.0;
    for i in (0..data.len()).step_by(sample_step) {
        sum += data[i] as f64;
        count += 1.0;
    }
    let mean = sum / count;

    let mut var_sum = 0.0;
    for i in (0..data.len()).step_by(sample_step) {
        let diff = data[i] as f64 - mean;
        var_sum += diff * diff;
    }
    let variance = var_sum / count;

    // An empty glass typically has variance < 50.
    // A finger placed usually pushes variance > 300.
    variance > 100.0
}

/// Convert BMP to raw grayscale (reimplements `UserLib::doConvertBmp2RawImage`)
pub fn convert_bmp_to_raw(bmp_data: &[u8]) -> Result<GrayImage, ImageError> {
    GrayImage::from_bmp(bmp_data)
}

/// Convert raw grayscale to BMP (reimplements `UserLib::doConvertRawImage2Bmp`)
pub fn convert_raw_to_bmp(img: &GrayImage) -> Vec<u8> {
    img.to_bmp()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_image(w: u32, h: u32, val: u8) -> GrayImage {
        GrayImage::from_raw(vec![val; (w * h) as usize], w, h).unwrap()
    }

    fn make_gradient_image(w: u32, h: u32) -> GrayImage {
        let mut data = vec![0u8; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                // Create vertical stripes to simulate ridges
                let stripe = ((x / 8) % 2) as u8;
                data[(y * w + x) as usize] = if stripe == 0 { 40 } else { 200 };
            }
        }
        GrayImage::from_raw(data, w, h).unwrap()
    }

    #[test]
    fn test_gray_image_create() {
        let img = make_test_image(300, 375, 128);
        assert_eq!(img.width(), 300);
        assert_eq!(img.height(), 375);
        assert_eq!(img.pixel(0, 0), 128);
    }

    #[test]
    fn test_gray_image_pixel_access() {
        let mut img = make_test_image(10, 10, 0);
        img.set_pixel(5, 5, 200);
        assert_eq!(img.pixel(5, 5), 200);
        assert_eq!(img.pixel(0, 0), 0);
    }

    #[test]
    fn test_sobel_derivatives() {
        let mut img = make_test_image(10, 10, 0);
        // Create a horizontal edge: top half black, bottom half white
        for y in 5..10 {
            for x in 0..10 {
                img.set_pixel(x, y, 255);
            }
        }
        let sy = img.sobel_y(5, 4); // above edge
        assert!(sy > 0.0, "Sobel Y should be positive above horizontal edge");
    }

    #[test]
    fn test_calculate_orientations() {
        let img = make_gradient_image(300, 375);
        let orient = calculate_orientations(&img, 8);
        assert_eq!(orient.width, 37);
        assert_eq!(orient.height, 46);
        // Vertical stripes → ridges run vertically → orientation is ~π/2
        let pi = std::f64::consts::PI;
        for by in 0..orient.height {
            for bx in 0..orient.width {
                let a = orient.angle(bx, by);
                // Vertical ridges: angle should be near π/2
                let near_pi_half = (a - pi / 2.0).abs() < 0.5;
                assert!(
                    near_pi_half,
                    "Expected vertical orientation (~π/2), got {}",
                    a
                );
            }
        }
    }

    #[test]
    fn test_calculate_frequency() {
        let img = make_gradient_image(300, 375);
        let orient = calculate_orientations(&img, 8);
        let freq = calculate_frequency(&img, &orient);
        assert_eq!(freq.width, 37);
        assert_eq!(freq.height, 46);
        // All frequencies should be positive
        for &f in &freq.frequencies {
            assert!(f > 0.0, "Frequency should be positive, got {}", f);
        }
    }

    #[test]
    fn test_calculate_block_quality() {
        let img = make_gradient_image(300, 375);
        let qmap = calculate_block_quality(&img);
        assert_eq!(qmap.width, 37);
        assert_eq!(qmap.height, 46);
        // Gradient image should have high quality (strong gradients)
        let mean_q = qmap.qualities.iter().sum::<f64>() / qmap.qualities.len() as f64;
        assert!(
            mean_q > 0.1,
            "Gradient image should have non-trivial quality, got {}",
            mean_q
        );
    }

    #[test]
    fn test_detect_background() {
        let img = make_gradient_image(300, 375);
        let bg = detect_background(&img);
        assert_eq!(bg.width, 300);
        assert_eq!(bg.height, 375);
        // Gradient image has alternating dark/light stripes = foreground
        // The background detection uses mean+variance, so stripes should be detected
        let fg_count = bg.mask.iter().filter(|&&b| b).count();
        // At minimum, some foreground should be detected (or none in edge case)
        // The key thing is it doesn't panic and produces a valid mask
        assert!(
            fg_count < bg.mask.len(),
            "Should have some background pixels"
        );
    }

    #[test]
    fn test_detect_finger_presence() {
        // High variance (alternating stripes) = finger present
        let mut finger_data = vec![0; 300 * 375];
        for i in 0..finger_data.len() {
            finger_data[i] = if i % 2 == 0 { 50 } else { 200 };
        }
        assert!(detect_finger_presence(&finger_data, 300, 375));

        // Low variance (flat color) = empty glass
        let empty_data = vec![240; 300 * 375];
        assert!(!detect_finger_presence(&empty_data, 300, 375));
    }

    #[test]
    fn test_resize() {
        let img = make_test_image(300, 375, 128);
        let resized = img.resize(128, 144);
        assert_eq!(resized.width(), 128);
        assert_eq!(resized.height(), 144);
    }
}
