use wasm_bindgen::prelude::*;
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use blake3::Hash;

// WASM module for offloading heavy crypto operations
#[wasm_bindgen]
pub struct BlockchainAnchor {
    keypair: Keypair,
}

#[wasm_bindgen]
impl BlockchainAnchor {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: Option<String>) -> Self {
        let keypair = if let Some(seed_str) = seed {
            // Generate deterministic keypair from seed
            let seed_bytes = blake3::hash(seed_str.as_bytes()).as_bytes().to_owned();
            let mut seed_array = [0u8; 32];
            seed_array.copy_from_slice(&seed_bytes[0..32]);
            Keypair::from_bytes(&seed_array).unwrap_or_else(|_| Keypair::generate(&mut rand::thread_rng()))
        } else {
            // Generate random keypair
            Keypair::generate(&mut rand::thread_rng())
        };

        Self { keypair }
    }
    
    #[wasm_bindgen]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.keypair.sign(message);
        signature.to_bytes().to_vec()
    }
    
    #[wasm_bindgen]
    pub fn verify(&self, signature: &[u8], message: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }
        
        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(signature);
        
        match Signature::from_bytes(&sig_bytes) {
            Ok(sig) => self.keypair.verify(message, &sig).is_ok(),
            Err(_) => false,
        }
    }
    
    #[wasm_bindgen]
    pub fn hash_data(&self, data: &[u8]) -> Vec<u8> {
        blake3::hash(data).as_bytes().to_vec()
    }
    
    #[wasm_bindgen]
    pub fn get_public_key(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}