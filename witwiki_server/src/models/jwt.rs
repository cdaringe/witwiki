use crate::models::role::Role;
use cookie::{time::Duration, Cookie, SameSite};
use jsonwebtoken::{
    decode as jwtdecode, encode as jwtencode, errors::Error, DecodingKey, EncodingKey, Header,
    Validation,
};
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

pub fn decode() -> () {
    let _ = jwtdecode::<Claims>(
        "token",
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    todo!()
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
