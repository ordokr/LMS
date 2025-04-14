use ed25519_dalek::{Keypair, Signature, Signer, Verifier};

pub fn sign_message(message: &[u8], keypair: &Keypair) -> Signature {
    keypair.sign(message)
}

pub fn verify_signature(message: &[u8], signature: &Signature, public_key: &ed25519_dalek::PublicKey) -> bool {
    public_key.verify_strict(message, signature).is_ok()
}