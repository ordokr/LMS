# Ordo Security Implementation

_Last updated: 2025-04-18_

## Overview

Ordo implements a robust security model using modern cryptographic practices, focusing on secure credential management, authentication, and data protection. This document outlines the security architecture and implementation details.

## Core Security Stack

```rust
// src-tauri/src/security/mod.rs
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{rand_core::OsRng, PasswordHash, SaltString};
use secrecy::{ExposeSecret, Secret};

pub struct CredentialManager {
    argon2: Argon2<'static>,
    pepper: Secret<String>,
}

impl CredentialManager {
    pub fn new(pepper: &str) -> Self {
        Self {
            argon2: Argon2::default(),
            pepper: Secret::new(pepper.to_string()),
        }
    }

    pub fn hash_password(&self, password: &Secret<String>) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self.argon2.hash_password(
            format!("{}{}", password.expose_secret(), self.pepper.expose_secret()).as_bytes(),
            &salt
        )?;
        Ok(hash.to_string())
    }

    pub fn verify_password(&self, password: &Secret<String>, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(self.argon2.verify_password(
            format!("{}{}", password.expose_secret(), self.pepper.expose_secret()).as_bytes(),
            &parsed_hash
        ).is_ok())
    }
}
```

## Implementation Strategy

### 1. Algorithm Selection

- **Primary**: Argon2id via the `argon2` crate
  - Memory-hard algorithm resistant to GPU/ASIC attacks
  - Recommended by OWASP and security experts

- **Fallback**: PBKDF2 via the `pbkdf2` crate
  - Used for compatibility with legacy systems when needed

- **Interoperability**: `password-hash` crate for standardized PHC strings
  - Ensures compatibility with other systems and future upgrades

### 2. Security Features

- **Peppering**: Application-wide secret combined with passwords
  - Stored outside the database for additional security layer
  - Mitigates database compromise scenarios

- **Secure Salting**: OsRng generated salts (16+ bytes)
  - Unique per password
  - Prevents rainbow table attacks

- **Memory Protection**: `secrecy` crate for zeroize-on-drop
  - Prevents passwords from lingering in memory
  - Mitigates memory dump attacks

- **Parameters**:

```rust
const ARGON2_PARAMS: Params = Params::new(
    19456,  // 19MB memory
    10,     // iterations
    1,      // parallelism
).unwrap();
```

### 3. Windows Optimization

```toml
# Cargo.toml
[features]
simd = ["argon2/simd"]  # Enable CPU-specific optimizations
```

## Storage Integration

```rust
// src-tauri/src/db/credential_store.rs
pub struct CredentialStore {
    db: HybridStore,
    manager: CredentialManager,
}

impl CredentialStore {
    pub async fn store_credentials(&self, user_id: &Uuid, password: &Secret<String>) -> Result<()> {
        let hash = self.manager.hash_password(password)?;
        self.db.execute(
            "INSERT INTO credentials (user_id, hash) VALUES (?, ?)",
            params![user_id, hash],
        ).await
    }
}
```

## Recommended Crates

> **Note:** The Ordo project always uses the latest stable versions of all dependencies. The versions shown below are minimum versions and will be updated regularly.

| Crate | Version | Purpose | License | Notes |
|-------|---------|---------|---------|-------|
| argon2 | 0.5.2+ | Argon2 implementation | MIT/Apache | RustCrypto maintained |
| password-hash | 0.5.0+ | PHC string format | MIT/Apache | Standardized hash serialization |
| secrecy | 0.8.0+ | Secret management | MIT | Zeroize-on-drop for sensitive data |
| rand_core | 0.6.4+ | Cryptographic RNG | MIT/Apache | OsRng implementation |

## Security Audit Checklist

- [x] **Parameter Tuning**: 19MB memory usage, 10 iterations, parallelism=1
- [x] **Pepper Storage**: Environment variable injection via Tauri config
- [x] **Logging Sanitization**: Automatic redaction of sensitive fields
- [x] **Timing Attack Protection**: Constant-time verification in argon2
- [x] **Credential Rotation**: Optional hash migration system

## OWASP Compliance

This implementation meets the OWASP Application Security Verification Standard (ASVS) requirements for secure password storage:

1. **Verified against ASVS v4.0.3 Section 2.4 (Authentication Verification Requirements)**
2. **Implements NIST SP 800-63B guidelines for memorized secrets**

## Offline-First Considerations

The security implementation is designed to work seamlessly in offline environments:

- **Local Verification**: Authentication can occur without network connectivity
- **Sync Protection**: Credentials are never synced in plaintext
- **Conflict Resolution**: Version vectors prevent credential conflicts

## Academic Environment Enhancements

For academic environments, the system includes:

- **Role-Based Access**: Fine-grained permissions for instructors, students, and administrators
- **Session Management**: Secure session handling with appropriate timeouts
- **Audit Logging**: Comprehensive logging of security-relevant events

## Future Enhancements

- **ZK-SNARK Proofs**: For password validity without exposing credentials
- **Hardware Security Module (HSM) Integration**: For enterprise deployments
- **Biometric Authentication**: For enhanced multi-factor options
