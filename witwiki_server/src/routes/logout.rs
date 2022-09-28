use crate::models::{api_response::ApiResponse, jwt::build_cookie};
use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
    Json,
};
use cookie::time::Duration;

pub async fn logout() -> impl IntoResponse {
    (
        StatusCode::OK,
        AppendHeaders([(
            SET_COOKIE,
            build_cookie(None, Duration::seconds(0)).to_string(),
        )]),
        Json(ApiResponse::new(vec![true], 1)),
    )
        .into_response()
}
