use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub port: u16,
    pub max_connections: usize,
    pub banner: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersistenceConfig {
    pub enabled: bool,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub persistence: PersistenceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                bind_addr: " ".to_string(), // TU IP PÚBLICA
                port: 4444, // MODIFICAR PUERTO
                max_connections: 10,
                banner: true,
            },
            persistence: PersistenceConfig {
                enabled: true,
                method: "startup".to_string(),
            },
        }
    }
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    if Path::new(path).exists() {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        println!("[!] Config no encontrada, usando valores por defecto");
        let config = Config::default();
        save_config(&config, path)?;
        Ok(config)
    }
}

pub fn save_config(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}