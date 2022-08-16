use crate::models::role::Role;
use jsonwebtoken::{encode as jwtencode, errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub roles: HashSet<Role>,
}

pub fn encode(claims: &Claims, secret: &str) -> Result<String, Error> {
    jwtencode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
