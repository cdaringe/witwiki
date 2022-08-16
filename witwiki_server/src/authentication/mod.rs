#![allow(dead_code, unused)]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params, Version,
};

#[derive(Debug, Clone)]
pub enum Role {
    Admin,
    ReadOnly,
    UserDefined(String),
}

#[derive(Debug, Clone)]
pub struct User {
    roles: Option<Vec<Role>>,
    username: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Authenticated {
    In,
    Out,
}

fn create_argon<'b>() -> Argon2<'b> {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    )
}

// pub fn create_password_idenity() {
//  let hashed = argon2
//  .hash_password(passbytes, salt)
//  .map_err(|err| err.to_string())?
//  .to_string();
// }

pub fn authenticate(password: &str, hash: &str) -> Result<Authenticated, String> {
    let argon2 = create_argon();
    let parsed_hash = PasswordHash::new(hash).map_err(|err| err.to_string())?;
    if argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(Authenticated::In)
    } else {
        Ok(Authenticated::Out)
    }
}
