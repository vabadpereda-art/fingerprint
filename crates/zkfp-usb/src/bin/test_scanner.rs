use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use rusb::UsbContext;
use zkfp_usb::{
    BULK_IN_ENDPOINT, CMD_CLR_IMAGE, CMD_DET_IMAGE, CMD_GET_GPIO,
    CMD_HANDSHAKE, CMD_INIT, CMD_SET_GPIO, CTRL_OUT, CTRL_IN, GPIO_ANTI_FAKE,
    GPIO_OUTPUT_MODE, GPIO_POWER_MODE, GPIO_WORK_MODE, MODE_DETECT,
    ZK9500_PID, ZKTECO_VID, Zk9500,
};

fn save_bmp_grayscale(filename: &str, data: &[u8], width: usize, height: usize) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let row_size = (width + 3) & !3; // Pad to multiple of 4
    let padding = vec![0u8; row_size - width];
    let img_size = row_size * height;
    let file_size = 54 + 1024 + img_size;

    // 1. BMP Header (14 bytes)
    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&[0, 0, 0, 0])?; // Reserved
    file.write_all(&(54u32 + 1024u32).to_le_bytes())?; // Offset to pixel data

    // 2. DIB Header / BITMAPINFOHEADER (40 bytes)
    file.write_all(&40u32.to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(-(height as i32)).to_le_bytes())?; // Top-down
    file.write_all(&1u16.to_le_bytes())?; // Planes
    file.write_all(&8u16.to_le_bytes())?; // 8 bpp
    file.write_all(&0u32.to_le_bytes())?; // BI_RGB (uncompressed)
    file.write_all(&(img_size as u32).to_le_bytes())?;
    file.write_all(&2835u32.to_le_bytes())?; // 500 DPI (~19.68 ppm)
    file.write_all(&2835u32.to_le_bytes())?;
    file.write_all(&256u32.to_le_bytes())?; // 256 colors
    file.write_all(&0u32.to_le_bytes())?;

    // 3. Color Palette (256 * 4 = 1024 bytes)
    for i in 0..256 {
        file.write_all(&[i as u8, i as u8, i as u8, 0])?;
    }

    // 4. Pixels
    for y in 0..height {
        let start = y * width;
        let end = std::cmp::min(start + width, data.len());
        if start < data.len() {
            file.write_all(&data[start..end])?;
            if end - start < width {
                file.write_all(&vec![0u8; width - (end - start)])?;
            }
        } else {
            file.write_all(&vec![0u8; width])?;
        }
        if !padding.is_empty() {
            file.write_all(&padding)?;
        }
    }

    println!("  -> BMP guardado: {} ({}x{})", filename, width, height);
    Ok(())
}

fn print_ascii_preview(data: &[u8], width: usize, height: usize) {
    println!("\n--- Vista Previa ASCII ({}x{}) ---", width, height);
    let sample_w = 40;
    let sample_h = 20;
    let step_x = (width / sample_w).max(1);
    let step_y = (height / sample_h).max(1);
    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

    for y in (0..height).step_by(step_y).take(sample_h) {
        let mut line = String::with_capacity(sample_w);
        for x in (0..width).step_by(step_x).take(sample_w) {
            let idx = y * width + x;
            let val = if idx < data.len() { data[idx] } else { 0 };
            let char_idx = (val as usize * (chars.len() - 1)) / 255;
            line.push(chars[char_idx]);
        }
        println!("|{}|", line);
    }
}

fn analyze_buffer(name: &str, data: &[u8]) {
    if data.is_empty() {
        println!("[{}] Buffer vacío (0 bytes)", name);
        return;
    }

    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();
    let sum: u64 = data.iter().map(|&x| x as u64).sum();
    let mean = sum as f64 / data.len() as f64;

    let var: f64 = data.iter().map(|&x| {
        let diff = x as f64 - mean;
        diff * diff
    }).sum::<f64>() / data.len() as f64;
    let std_dev = var.sqrt();

    println!("\n[{}] Estadísticas de imagen:", name);
    println!("  Total bytes: {}", data.len());
    println!("  Min: {} | Max: {} | Media: {:.2} | Desv. Estándar: {:.2}", min, max, mean, std_dev);

    if std_dev < 5.0 {
        println!("  ⚠️ ALERTA: La imagen es prácticamente uniforme / plana (posible sensor ciego, LED apagado o buffer vacío)");
    } else {
        println!("  ✅ Variación detectada: contiene contraste visual de huella");
    }
}

fn main() {
    println!("==================================================");
    println!("     ZK9500 HERRAMIENTA DE DIAGNÓSTICO USB        ");
    println!("==================================================");

    // 1. Abrir con rusb directo para control total
    let context = match rusb::Context::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[ERROR] No se pudo inicializar rusb: {:?}", e);
            return;
        }
    };

    let handle = match context.open_device_with_vid_pid(ZKTECO_VID, ZK9500_PID) {
        Some(h) => h,
        None => {
            eprintln!("[ERROR] Sensor ZK9500 no encontrado (VID=0x1B55, PID=0x0124)");
            eprintln!("        Comprueba la conexión USB y permisos (udev/root).");
            return;
        }
    };

    println!("[OK] Dispositivo USB ZK9500 abierto.");

    // Claim interface 0
    if handle.kernel_driver_active(0).unwrap_or(false) {
        let _ = handle.detach_kernel_driver(0);
    }
    if let Err(e) = handle.claim_interface(0) {
        eprintln!("[WARN] No se pudo reclamar interfaz 0: {:?}", e);
    } else {
        println!("[OK] Interfaz 0 reclamada.");
    }

    // 2. Drenar endpoints
    println!("\n--- Drenando buffers previos ---");
    let mut drain_buf = [0u8; 16384];
    for i in 0..5 {
        match handle.read_bulk(BULK_IN_ENDPOINT, &mut drain_buf, Duration::from_millis(50)) {
            Ok(n) => println!("  Drenados {} bytes de bulk", n),
            Err(_) => {
                if i == 0 {
                    println!("  Endpoint bulk limpio.");
                }
                break;
            }
        }
    }

    // 3. Inicialización del hardware
    println!("\n--- Enviando secuencia de inicialización ---");
    // CMD_INIT (0xE0)
    let _ = handle.write_control(CTRL_OUT, CMD_INIT, 0, 0, &[], Duration::from_millis(500));
    std::thread::sleep(Duration::from_millis(100));

    // CMD_HANDSHAKE (0x80)
    let handshake_payload = [0u8; 16];
    let _ = handle.write_control(CTRL_OUT, CMD_HANDSHAKE, 0, 0, &handshake_payload, Duration::from_millis(500));
    let mut hs_buf = vec![0u8; 16384];
    let _ = handle.read_bulk(BULK_IN_ENDPOINT, &mut hs_buf, Duration::from_millis(500));

    // Get firmware version
    let mut fw_buf = [0u8; 4];
    let _ = handle.read_control(CTRL_IN, CMD_GET_GPIO, 0, 0x55, &mut fw_buf, Duration::from_millis(500));
    println!("[INFO] Firmware version: {}.{}", fw_buf[0], fw_buf[1]);

    // Configurar GPIOs
    let _ = handle.write_control(CTRL_OUT, CMD_SET_GPIO, 1, GPIO_POWER_MODE, &[], Duration::from_millis(500));
    let _ = handle.write_control(CTRL_OUT, CMD_SET_GPIO, 1, GPIO_ANTI_FAKE, &[], Duration::from_millis(500));
    let _ = handle.write_control(CTRL_OUT, CMD_SET_GPIO, 1, GPIO_OUTPUT_MODE, &[], Duration::from_millis(500));
    let _ = handle.write_control(CTRL_OUT, CMD_SET_GPIO, MODE_DETECT, GPIO_WORK_MODE, &[], Duration::from_millis(500));
    
    // Clear Halt and Image buffer
    let _ = handle.write_control(CTRL_OUT, CMD_CLR_IMAGE, 0, 0, &[], Duration::from_millis(500));
    let _ = handle.clear_halt(BULK_IN_ENDPOINT);

    println!("\n==================================================");
    println!(">>> COLOCA EL DEDO EN EL SENSOR PARA CAPTURAR <<<");
    println!("==================================================");

    // 4. Polling de detección de dedo con CMD_DET_IMAGE
    let start_detect = Instant::now();
    let mut finger_detected = false;
    while start_detect.elapsed() < Duration::from_secs(15) {
        let mut det_buf = [0u8; 10];
        match handle.read_control(CTRL_IN, CMD_DET_IMAGE, 0, 0, &mut det_buf, Duration::from_millis(200)) {
            Ok(n) if n >= 1 => {
                if det_buf[0] == 1 {
                    println!("\n[OK] ¡Dedo detectado! (status[0] = 0x01)");
                    finger_detected = true;
                    break;
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("[WARN] Error leyendo detección: {:?}", e);
            }
        }
        print!(".");
        std::io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }

    if !finger_detected {
        println!("\n[TIMEOUT] No se detectó ningún dedo en 15 segundos.");
        return;
    }

    // 5. Captura de datos vía Bulk IN
    println!("\n--- Leyendo stream de imagen desde Bulk IN (EP 0x82) ---");
    let mut total_data = Vec::new();
    let mut chunk_buf = vec![0u8; 16384];
    let start_read = Instant::now();
    let mut chunk_count = 0;

    while start_read.elapsed() < Duration::from_secs(4) {
        match handle.read_bulk(BULK_IN_ENDPOINT, &mut chunk_buf, Duration::from_millis(500)) {
            Ok(n) if n > 0 => {
                chunk_count += 1;
                total_data.extend_from_slice(&chunk_buf[..n]);
                println!("  Chunk #{}: +{} bytes (Total acumulado: {} bytes)", chunk_count, n, total_data.len());
                // Si llegamos a los tamaños estándar conocidos (112500 o 108000), dar una pequeña oportunidad de completar
                if total_data.len() >= 112500 {
                    break;
                }
            }
            Ok(_) => {}
            Err(rusb::Error::Timeout) => {
                if !total_data.is_empty() {
                    println!("  [Fin de transmisión por timeout del sensor]");
                    break;
                }
            }
            Err(rusb::Error::Overflow) | Err(rusb::Error::Pipe) => {
                let _ = handle.clear_halt(BULK_IN_ENDPOINT);
                if total_data.len() >= 100000 {
                    break;
                }
            }
            Err(rusb::Error::Io) => {
                let _ = handle.clear_halt(BULK_IN_ENDPOINT);
                if total_data.len() >= 100000 {
                    break;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                eprintln!("  [ERROR en bulk_read]: {:?}", e);
                break;
            }
        }
    }

    // 6. Análisis y guardado de los datos capturados
    println!("\n==================================================");
    println!("             RESULTADOS DE LA CAPTURA             ");
    println!("==================================================");
    println!("Total de bytes recibidos: {}", total_data.len());

    if total_data.is_empty() {
        println!("[FALLO] No se recibieron bytes de imagen.");
        return;
    }

    // Guardar raw binario
    let raw_filename = "debug_capture.raw";
    if let Ok(mut f) = File::create(raw_filename) {
        let _ = f.write_all(&total_data);
        println!("  -> Archivo RAW guardado: {} ({} bytes)", raw_filename, total_data.len());
    }

    analyze_buffer("DUMP_RAW", &total_data);

    // Intentar interpretar con diferentes geometrías
    let candidates = [
        ("captura_300x375.bmp", 300, 375), // Estándar SDK (112,500)
        ("captura_300x360.bmp", 300, 360), // Sensor nativo (108,000)
        ("captura_288x375.bmp", 288, 375), // Alternativa (108,000)
        ("captura_256x360.bmp", 256, 360),
    ];

    for (name, w, h) in candidates {
        let expected = w * h;
        if total_data.len() >= expected {
            let _ = save_bmp_grayscale(name, &total_data[..expected], w, h);
        } else if total_data.len() > 0 {
            // Padding si faltan bytes
            let mut padded = total_data.clone();
            padded.resize(expected, 0);
            let _ = save_bmp_grayscale(name, &padded, w, h);
        }
    }

    // Mostrar preview según la resolución detectada
    let (preview_w, preview_h) = if total_data.len() >= 112500 {
        (300, 375)
    } else if total_data.len() >= 108000 {
        (288, 375)
    } else {
        (300, (total_data.len() / 300).max(1))
    };
    if preview_h > 10 {
        print_ascii_preview(&total_data, preview_w, preview_h);
    }

    // 7. Test secundario: Probar también la API de alto nivel `zkfp_usb::Zk9500`
    println!("\n==================================================");
    println!("     PROBANDO CAPTURA CON LIB `zkfp_usb`         ");
    println!("==================================================");
    drop(handle); // Liberar el handle directo antes de abrir con la clase
    std::thread::sleep(Duration::from_millis(200));

    match Zk9500::open() {
        Ok(mut dev) => {
            match dev.init() {
                Ok(fw) => {
                    println!("[OK] Zk9500::init() completado. Firmware: {}", fw);
                    println!(">>> Pon el dedo para prueba con Zk9500::capture_image() <<<");
                    match dev.capture_image() {
                        Ok(img) => {
                            println!("[OK] ¡Zk9500::capture_image() tuvo ÉXITO!");
                            println!("     Dimensiones: {}x{}, DPI: {}, Bytes: {}", img.width, img.height, img.dpi, img.data.len());
                            let _ = save_bmp_grayscale("captura_zk9500_api.bmp", &img.data, img.width as usize, img.height as usize);
                        }
                        Err(e) => {
                            eprintln!("[ERROR en dev.capture_image()]: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[ERROR en dev.init()]: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR en Zk9500::open()]: {:?}", e);
        }
    }

    println!("\n[DIAGNÓSTICO FINALIZADO]");
}
