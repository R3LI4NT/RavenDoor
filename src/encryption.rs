use aes::Aes256;
use cbc::{Encryptor, Decryptor};
use cipher::{
    BlockEncryptMut,
    BlockDecryptMut,
    KeyIvInit,
    generic_array::GenericArray,
};
use rand::RngCore;

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

// Colores para logs internos
const RESET: &str = "\x1b[0m";
const BRIGHT_CYAN: &str = "\x1b[96m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const BRIGHT_RED: &str = "\x1b[91m";
const BRIGHT_YELLOW: &str = "\x1b[93m";

pub struct Encryption {
    key: [u8; 32],
    iv: [u8; 16],
}

impl Encryption {
    pub fn new(key: &[u8; 32]) -> Self {
        let mut iv = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut iv);

        println!("{}[ENCRYPTION] Nuevo cifrador CBC creado{}", BRIGHT_CYAN, RESET);
        println!("{}[ENCRYPTION] IV generado: {}{}", BRIGHT_GREEN, hex::encode(iv), RESET);
        println!("{}[ENCRYPTION] Clave: {}...{}{}", BRIGHT_GREEN, 
                &hex::encode(key)[..16], &hex::encode(key)[48..], RESET);

        Self {
            key: *key,
            iv,
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, String> {
        println!("{}[ENCRYPTION] Encriptando texto ({} bytes){}", 
                BRIGHT_CYAN, plaintext.len(), RESET);
        
        let key = GenericArray::from_slice(&self.key);
        let iv = GenericArray::from_slice(&self.iv);

        // 👇 DEBE SER MUT
        let mut cipher = Aes256CbcEnc::new(key, iv);

        let mut buffer = plaintext.as_bytes().to_vec();

        // PKCS7 padding
        let block_size = 16;
        let padding_len = block_size - (buffer.len() % block_size);
        buffer.extend(std::iter::repeat(padding_len as u8).take(padding_len));

        println!("{}[ENCRYPTION] Buffer con padding: {} bytes{}", 
                BRIGHT_YELLOW, buffer.len(), RESET);

        for block in buffer.chunks_mut(block_size) {
            let block = GenericArray::from_mut_slice(block);
            cipher.encrypt_block_mut(block);
        }

        println!("{}[ENCRYPTION] Encriptación exitosa: {} bytes{}", 
                BRIGHT_GREEN, buffer.len(), RESET);
        
        Ok(buffer)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<String, String> {
        if ciphertext.len() % 16 != 0 {
            let error = format!("Ciphertext no es múltiplo del bloque AES: {} bytes", ciphertext.len());
            println!("{}[ENCRYPTION] ERROR: {}{}", BRIGHT_RED, error, RESET);
            return Err(error.into());
        }

        println!("{}[ENCRYPTION] Desencriptando {} bytes{}", 
                BRIGHT_CYAN, ciphertext.len(), RESET);

        let key = GenericArray::from_slice(&self.key);
        let iv = GenericArray::from_slice(&self.iv);

        // 👇 DEBE SER MUT
        let mut cipher = Aes256CbcDec::new(key, iv);

        let mut buffer = ciphertext.to_vec();

        for block in buffer.chunks_mut(16) {
            let block = GenericArray::from_mut_slice(block);
            cipher.decrypt_block_mut(block);
        }

        // Quitar PKCS7 padding
        let padding_len = match buffer.last() {
            Some(&len) => len as usize,
            None => {
                println!("{}[ENCRYPTION] ERROR: Buffer vacío{}", BRIGHT_RED, RESET);
                return Err("Buffer vacío".into());
            }
        };

        if padding_len == 0 || padding_len > 16 || padding_len > buffer.len() {
            let error = format!("Padding inválido: {}", padding_len);
            println!("{}[ENCRYPTION] ERROR: {}{}", BRIGHT_RED, error, RESET);
            return Err(error.into());
        }

        buffer.truncate(buffer.len() - padding_len);

        println!("{}[ENCRYPTION] Desencriptación exitosa: {} bytes{}", 
                BRIGHT_GREEN, buffer.len(), RESET);

        String::from_utf8(buffer)
            .map_err(|e| {
                let error = format!("UTF-8 error: {}", e);
                println!("{}[ENCRYPTION] ERROR: {}{}", BRIGHT_RED, error, RESET);
                error
            })
    }

    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        println!("{}[ENCRYPTION] Clave generada: {}...{}{}", 
                BRIGHT_GREEN, &hex::encode(key)[..16], &hex::encode(key)[48..], RESET);
        key
    }

    pub fn get_iv(&self) -> &[u8; 16] {
        &self.iv
    }

    pub fn get_iv_hex(&self) -> String {
        hex::encode(self.iv)
    }

    pub fn key_to_hex(key: &[u8; 32]) -> String {
        hex::encode(key)
    }

    pub fn hex_to_key(hex_str: &str) -> Result<[u8; 32], String> {
        println!("{}[ENCRYPTION] Convirtiendo hex a clave{}", BRIGHT_CYAN, RESET);
        
        let bytes = hex::decode(hex_str)
            .map_err(|e| {
                let error = format!("Error decode hex: {}", e);
                println!("{}[ENCRYPTION] ERROR: {}{}", BRIGHT_RED, error, RESET);
                error
            })?;

        if bytes.len() != 32 {
            let error = "La clave debe tener 32 bytes (64 hex chars)".to_string();
            println!("{}[ENCRYPTION] ERROR: {} (tenía {} bytes){}", 
                    BRIGHT_RED, error, bytes.len(), RESET);
            return Err(error.into());
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(&bytes);
        
        println!("{}[ENCRYPTION] Clave convertida exitosamente{}", BRIGHT_GREEN, RESET);
        Ok(key)
    }
}