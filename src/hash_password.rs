use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString, PasswordHash},
    Argon2,
};
use rand_core::OsRng;


/// Hashes a password using Argon2 and a random salt.
/// Returns a string you can store in the database.
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Hashing failed: {}", e))
}

/// Verifies a password against a stored Argon2 hash.
pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::default();
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok(),
        Err(_) => false,
    }
}
