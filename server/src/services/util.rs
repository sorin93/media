use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{Error, Result};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn generate_code() -> (String, String) {
    let code: String = (0..8)
        .map(|_| {
            let i = rand::random_range(0..CHARSET.len());
            CHARSET[i] as char
        })
        .collect();
    let hash = hash(&code);
    println!("{} {}", code, hash); // todo
    (code, hash)
}

pub fn generate_token() -> (String, String) {
    let bytes: [u8; 32] = rand::random();
    let token = URL_SAFE_NO_PAD.encode(bytes);
    let hash = hash(&token);
    (token, hash)
}

pub fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(result)
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub fn validate_country(
    country: &str,
) -> Result<()> {
    const COUNTRIES: &[&str] = &["CA", "US"];
    let country_upper = country.to_uppercase();
    if COUNTRIES.contains(&country_upper.as_str()) {
        Ok(())
    } else {
        Err(Error::UnprocessableEntity("Invalid country".into()))
    }
}

pub fn validate_email(
    email: &mut String,
) -> Result<()> {
    // todo: regex
    validate_string(email, 10, 50, "Invalid e-mail")
}

pub fn validate_number<T: PartialOrd>(
    val: T,
    min_val: T,
    max_val: T,
    message: &str,
) -> Result<()> {
    if val < min_val || val > max_val {
        return Err(Error::UnprocessableEntity(message.into()));
    }
    Ok(())
}

pub fn validate_number_opt<T: PartialOrd>(
    val: Option<T>,
    min_val: T,
    max_val: T,
    message: &str,
) -> Result<()> {
    if let Some(val) = val {
        if val < min_val || val > max_val {
            return Err(Error::UnprocessableEntity(message.into()));
        }
    }
    Ok(())
}

pub fn validate_password(
    password: &mut String,
) -> Result<()> {
    // todo: regex
    validate_string(password, 8, 30, "Invalid password")
}

pub fn validate_string(
    str: &mut String,
    min_len: usize,
    max_len: usize,
    message: &str,
) -> Result<()> {
    *str = str.trim().to_string();
    let len = str.len();
    if len < min_len || len > max_len {
       return Err(Error::UnprocessableEntity(message.into()));
    }
    Ok(())
}

pub fn validate_string_opt(
    str: &mut Option<String>,
    min_len: usize,
    max_len: usize,
    message: &str,
) -> Result<()> {
    if let Some(str) = str {
        *str = str.trim().to_string();
        let len = str.len();
        if len < min_len || len > max_len {
            return Err(Error::UnprocessableEntity(message.into()));
        }
    }
    Ok(())
}