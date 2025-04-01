use argon2::{
    Argon2,
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
