use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash_password(plaintext: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = argon2
        .hash_password(plaintext.as_bytes(), &salt)?
        .to_string();
    Ok(hashed)
}

/// Hashes a provided plaintext password and compares it to a known hash. Returns `true` or `false` based on correct/incorrect passwords.
pub fn validate_password(
    plaintext: &str,
    hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash).expect("Failed to hash password.");
    let argon2 = Argon2::default();
    match argon2.verify_password(plaintext.as_bytes(), &parsed) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}
