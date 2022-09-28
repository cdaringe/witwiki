use crate::{
    authentication,
    authentication::Authenticated,
    dao::auth::query_unpw_identity,
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
use std::collections::HashSet;

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
    let user_result: Result<User, _> = sqlx::query_as!(
        User,
        r"
select
  id,
  username,
  first_name,
  last_name,
  user_preferences_id,
  authentication_strategy
from user
where username = ?
",
        body.username
    )
    .fetch_one(&mut pool)
    .await;
    if let Ok(user) = user_result
      && user.authentication_strategy == 1
      && let Ok(identity) =  query_unpw_identity(user.id, &mut pool).await
      {
          match authentication::authenticate(&body.password, &identity.hash) {
            Ok(auth_state) => {
              if auth_state == Authenticated::In {
                let duration = Duration::days(1);
                let exp = duration.as_seconds_f64() as usize;
                let session_jwt = encode(&Claims {
                  sub: "".to_string(),
                  exp,
                  roles: HashSet::new()
                }, "@todo").unwrap();
                let jwt_cookie = build_cookie(Some(session_jwt), duration);
                return (
                  StatusCode::OK,
                  AppendHeaders([(SET_COOKIE, jwt_cookie.to_string())]),
                  Json(ApiResponse::new(vec![user], 10))
                ).into_response();
              }
            },
            Err(v) => {
              println!("unexpected authorization failure: {}.\nare DB records invalid?", v);
              return (StatusCode::INTERNAL_SERVER_ERROR, Err::<(), &str>("500")).into_response()
            }
          }

  }
    (StatusCode::UNAUTHORIZED, Err::<(), &str>("409")).into_response()
}
