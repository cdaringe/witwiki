#![allow(dead_code, unused)]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params, Version,
};

fn create_argon<'b>() -> Argon2<'b> {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
}

pub fn authenticate(password: &str, hash: &str) -> Result<bool, String> {
    let argon2 = create_argon();
    let parsed_hash = PasswordHash::new(hash).map_err(|err| err.to_string())?;
    if argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(true)
    } else {
        Ok(false)
    }
}
