use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

pub fn generate_code() -> (String, String) {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
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
