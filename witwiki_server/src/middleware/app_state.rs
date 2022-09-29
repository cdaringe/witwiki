use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    Extension,
};

use witwiki_db::Db;

use crate::models::user::User;

#[derive(Clone, Debug)]
pub struct RequestState {
    _is_authenticated: Option<bool>,
    // _user: Option<User>,
    pub db: Arc<Db>,
}

impl RequestState {
    pub fn new(db: Arc<Db>) -> Self {
        RequestState {
            db,
            // _user: None,
            _is_authenticated: None,
        }
    }

    pub fn set_authenticated(self: &mut Self, x: bool) {
        self._is_authenticated = Some(x);
    }

    pub fn is_authenticated(self: &mut Self) -> bool {
        match self._is_authenticated {
            Some(x) => x,
            None => {
                let is_authenticated = true;
                tracing::info!("@todo not real");
                self._is_authenticated = Some(is_authenticated);
                is_authenticated
            }
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
