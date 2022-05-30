use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, RequestParts},
    http::{Request, StatusCode},
    Extension,
};
use cookie::Cookie;
use std::collections::HashMap;

use crate::authentication::LoggedIn;

type CookieMap = HashMap<String, Cookie<'static>>;

#[derive(Debug, Clone)]
pub struct RequestState {
    cookies_by_name: CookieMap,
    user: Option<LoggedIn>,
}

impl RequestState {
    pub fn default() -> Self {
        RequestState {
            cookies_by_name: HashMap::new(),
            user: None,
        }
    }
    pub fn get_cookies(&self) -> &CookieMap {
        &self.cookies_by_name
    }

    pub fn set_cookies(&mut self, cookies_by_name: CookieMap) {
        self.cookies_by_name = cookies_by_name;
    }
}

#[async_trait]
impl<B> FromRequest<B> for RequestState
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(rs) = Extension::<RequestState>::from_request(req)
            .await
            .expect("extension missing");
        Ok(rs)
    }
}
