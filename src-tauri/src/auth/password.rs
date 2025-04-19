use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use anyhow::{Result, Context};

/// Hash a password using Argon2id
///
/// # Arguments
/// * `password` - The password to hash
///
/// # Returns
/// A Result containing the password hash as a string or an error
pub fn hash_password(password: &str) -> Result<String> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);
    
    // Create Argon2 instance with secure parameters
    let argon2 = Argon2::default();
    
    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .context("Failed to hash password")?
        .to_string();
    
    Ok(password_hash)
}

/// Verify a password against a hash
///
/// # Arguments
/// * `password` - The password to verify
/// * `hash` - The password hash to verify against
///
/// # Returns
/// A Result containing a boolean indicating whether the password is valid or an error
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // Parse the password hash
    let parsed_hash = PasswordHash::new(hash)
        .context("Failed to parse password hash")?;
    
    // Verify the password
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    
    // Return true if verification succeeded, false otherwise
    Ok(result.is_ok())
}

/// Check if a password meets the minimum requirements
///
/// # Arguments
/// * `password` - The password to check
///
/// # Returns
/// A Result containing a boolean indicating whether the password meets the requirements or an error message
pub fn validate_password_strength(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }
    
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if !has_uppercase {
        return Err("Password must contain at least one uppercase letter".to_string());
    }
    
    if !has_lowercase {
        return Err("Password must contain at least one lowercase letter".to_string());
    }
    
    if !has_digit {
        return Err("Password must contain at least one digit".to_string());
    }
    
    if !has_special {
        return Err("Password must contain at least one special character".to_string());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_hashing_and_verification() {
        let password = "SecurePassword123!";
        
        let hash = hash_password(password).expect("Failed to hash password");
        
        let is_valid = verify_password(password, &hash).expect("Failed to verify password");
        assert!(is_valid, "Password verification failed");
        
        let is_invalid = verify_password("WrongPassword", &hash).expect("Failed to verify password");
        assert!(!is_invalid, "Password verification should have failed");
    }
    
    #[test]
    fn test_password_strength_validation() {
        // Valid password
        assert!(validate_password_strength("SecurePassword123!").is_ok());
        
        // Too short
        assert!(validate_password_strength("Short1!").is_err());
        
        // No uppercase
        assert!(validate_password_strength("securepassword123!").is_err());
        
        // No lowercase
        assert!(validate_password_strength("SECUREPASSWORD123!").is_err());
        
        // No digit
        assert!(validate_password_strength("SecurePassword!").is_err());
        
        // No special character
        assert!(validate_password_strength("SecurePassword123").is_err());
    }
}
