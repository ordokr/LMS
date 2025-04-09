use wasm_bindgen::prelude::*;
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};

#[wasm_bindgen]
pub struct BlockchainAnchor {
    keypair: Keypair,
}

#[wasm_bindgen]
impl BlockchainAnchor {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: Option<String>) -> Self {
        // Implementation as before
        Self { keypair: Keypair::generate(&mut rand::thread_rng()) }
    }
    
    // Cryptographic operations as before
    #[wasm_bindgen]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.keypair.sign(message);
        signature.to_bytes().to_vec()
    }
    
    // Other methods as before
}