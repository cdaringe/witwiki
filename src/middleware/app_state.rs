use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, RequestParts},
    http::{Request, StatusCode},
    Extension,
};
use cookie::Cookie;
use std::collections::HashMap;

pub type Req = Request<Body>;

#[derive(Debug, Clone)]
pub struct RequestState {
    pub cookies_by_name: HashMap<String, Cookie<'static>>,
}

impl RequestState {
    pub fn default() -> Self {
        RequestState {
            cookies_by_name: HashMap::new(),
        }
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
