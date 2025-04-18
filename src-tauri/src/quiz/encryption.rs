use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use argon2::{self, Config};
use rand::{thread_rng, RngCore};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub struct ContentEncryption {
    key: Key<Aes256Gcm>,
    salt: [u8; 16],
}

impl ContentEncryption {
    pub fn new(encryption_key: Option<&str>) -> Result<Self> {
        let mut salt = [0u8; 16];
        thread_rng().fill_bytes(&mut salt);
        
        let key = match encryption_key {
            Some(key) => Self::derive_key(key, &salt)?,
            None => {
                let random_key = thread_rng().gen::<[u8; 32]>();
                Key::<Aes256Gcm>::from_slice(&random_key).clone()
            }
        };

        Ok(Self { key, salt })
    }

    pub fn encrypt_quiz(&self, quiz: &Quiz) -> Result<Vec<u8>> {
        let plaintext = serde_json::to_vec(quiz)?;
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(&self.generate_nonce());
        
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
            
        Ok(ciphertext)
    }

    pub fn decrypt_quiz(&self, ciphertext: &[u8]) -> Result<Quiz> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(&self.generate_nonce());
        
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
            
        let quiz = serde_json::from_slice(&plaintext)?;
        Ok(quiz)
    }

    fn derive_key(password: &str, salt: &[u8]) -> Result<Key<Aes256Gcm>> {
        let config = Config::default();
        let hash = argon2::hash_raw(
            password.as_bytes(),
            salt,
            &config,
        )?;
        
        Ok(Key::<Aes256Gcm>::from_slice(&hash[..32]).clone())
    }

    fn generate_nonce(&self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        thread_rng().fill_bytes(&mut nonce);
        nonce
    }
}