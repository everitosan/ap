use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngExt;

const NONCE_SIZE: usize = 12;

#[derive(Clone)]
pub struct CookieEncryptor {
    cipher: Aes256Gcm,
}

impl CookieEncryptor {
    pub fn new(secret: &str) -> Self {
        let mut key = [0u8; 32];
        let secret_bytes = secret.as_bytes();
        let len = secret_bytes.len().min(32);
        key[..len].copy_from_slice(&secret_bytes[..len]);

        let cipher = Aes256Gcm::new_from_slice(&key).expect("Invalid key length");
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let nonce_bytes: [u8; NONCE_SIZE] = rand::rng().random();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .expect("Encryption failed");

        let mut combined = nonce_bytes.to_vec();
        combined.extend(ciphertext);

        URL_SAFE_NO_PAD.encode(&combined)
    }

    pub fn decrypt(&self, encoded: &str) -> Option<String> {
        let combined = URL_SAFE_NO_PAD.decode(encoded).ok()?;

        if combined.len() < NONCE_SIZE {
            return None;
        }

        let (nonce_bytes, ciphertext) = combined.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self.cipher.decrypt(nonce, ciphertext).ok()?;
        String::from_utf8(plaintext).ok()
    }
}
