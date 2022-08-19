use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    Extension,
};

use witwiki_db::Db;

use crate::authentication::Authenticated;

#[derive(Clone, Debug)]
pub struct RequestState {
    pub user: Option<Authenticated>,
    pub db: Arc<Db>,
}

impl RequestState {
    pub fn new(db: Arc<Db>) -> Self {
        RequestState { db, user: None }
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
