use crate::models::role::Role;
use cookie::{time::Duration, Cookie, SameSite};
use jsonwebtoken::{encode as jwtencode, errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const COOKIE_NAME: &str = "jwt";
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

pub fn build_cookie<'a>(jwt: Option<String>, duration: Duration) -> Cookie<'a> {
    let value = match jwt {
        Some(jwt_str) => jwt_str,
        None => String::new(),
    };
    Cookie::build(COOKIE_NAME, value)
        .path("/")
        .http_only(true)
        .max_age(duration)
        .same_site(SameSite::Strict)
        .finish()
}