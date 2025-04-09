use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use blake3::Hasher;

#[deny(unsafe_code)]
pub struct BlockchainCrypto {
    keypair: Keypair,
}

impl BlockchainCrypto {
    pub fn new() -> Self {
        let keypair = Keypair::generate(&mut OsRng);
        Self { keypair }
    }
    
    pub fn from_seed(seed: &[u8]) -> Result<Self, ed25519_dalek::SignatureError> {
        // Securely derive keypair from seed
        let mut hasher = Hasher::new();
        hasher.update(seed);
        let hash = hasher.finalize();
        
        let secret = ed25519_dalek::SecretKey::from_bytes(hash.as_bytes())?;
        let public = ed25519_dalek::PublicKey::from(&secret);
        let keypair = Keypair { secret, public };
        
        Ok(Self { keypair })
    }
    
    pub fn sign(&self, data: &[u8]) -> Signature {
        self.keypair.sign(data)
    }
    
    pub fn verify(&self, signature: &Signature, data: &[u8]) -> bool {
        self.keypair.verify(data, signature).is_ok()
    }
    
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.keypair.public.to_bytes()
    }
}

// Optimized hash functions module - the only place with unsafe code
mod optimized_hash {
    #[allow(unsafe_code)]
    pub unsafe fn simd_optimized_hash(data: &[u8]) -> [u8; 32] {
        // This would contain carefully reviewed SIMD optimizations
        // Only allow unsafe code in this specific module
        let mut output = [0u8; 32];
        // SIMD implementation would go here
        output
    }
}