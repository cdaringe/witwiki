use crate::{
    authentication,
    dao::{
        auth::query_unpw_identity,
        user::{get_user, UserQuery},
    },
    error::{db_err, e400, e409, e500},
    models::{
        api_response::ApiResponse,
        jwt::{build_cookie, encode, Claims},
        user::User,
    },
    RequestState,
};
use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Extension, Json,
};
use cookie::time::Duration;
use serde::Deserialize;
use sqlx::Error;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

#[derive(Deserialize)]
pub struct AuthenticationUnPwBody {
    password: String,
    username: String,
}

/**
 * Login.
 * @example
curl -X POST -H "Content-Type: application/json" \
  -d '{"username": "raptorboy", "password": "password"}' \
  http://localhost:9999/api/login
*/
pub async fn login(
    body: Json<AuthenticationUnPwBody>,
    request_state: Extension<RequestState>,
) -> impl IntoResponse {
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let user = match get_user(UserQuery::from_username(&body.username), &mut pool).await {
        Ok(u) => u,
        Err(e) => return db_err(e).into_response(),
    };
    if user.authentication_strategy != 1 {
        return (
            StatusCode::BAD_REQUEST,
            Err::<(), &str>("unsupported strategy"),
        )
            .into_response();
    }
    match query_unpw_identity(user.id, &mut pool).await {
        Ok(identity) => match authentication::authenticate(&body.password, &identity.hash) {
            Ok(is_authenticated) => {
                if !is_authenticated {
                    return e409("").into_response();
                }
                let duration = Duration::days(1);
                let exp = duration.as_seconds_f64() as usize;
                let session_jwt = encode(
                    &Claims {
                        sub: "".to_string(),
                        exp,
                        roles: HashSet::new(),
                    },
                    "@todo",
                )
                .unwrap();
                let jwt_cookie = build_cookie(Some(session_jwt), duration);
                return (
                    StatusCode::OK,
                    AppendHeaders([(SET_COOKIE, jwt_cookie.to_string())]),
                    Json(ApiResponse::new(vec![user], 10)),
                )
                    .into_response();
            }
            Err(v) => return e500(&v, "").into_response(),
        },
        Err(e) => return e409(&format!("{:?}", e)).into_response(),
    }
}
