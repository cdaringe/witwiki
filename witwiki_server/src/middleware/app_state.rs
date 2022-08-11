#![allow(dead_code, unused)]

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    Extension,
};
use cookie::Cookie;
use std::{collections::HashMap, sync::Arc};
use witwiki_db::Db;

use crate::authentication::LoggedIn;

type CookieMap = HashMap<String, Cookie<'static>>;

#[derive(Clone, Debug)]
pub struct RequestState {
    cookies_by_name: CookieMap,
    user: Option<LoggedIn>,
    pub db: Arc<Db>,
}

impl RequestState {
    pub fn new(db: Arc<Db>) -> Self {
        RequestState {
            cookies_by_name: HashMap::new(),
            db,
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
