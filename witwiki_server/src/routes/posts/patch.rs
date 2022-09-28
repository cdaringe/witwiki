use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use sqlx;

use crate::{models::api_response::ApiResponse, RequestState};

#[derive(Deserialize)]
pub struct PatchQuery {
    id: i64,
    body: String,
    title: String,
    change_description: Option<String>,
}

#[derive(Deserialize)]
pub struct BodyRecord {
    body: String,
}

pub async fn patch(
    Json(model): Json<PatchQuery>,
    request_state: Extension<RequestState>,
) -> impl IntoResponse {
    let user = match &request_state.user {
        Some(u) => u,
        None => return (StatusCode::FORBIDDEN, Json(ApiResponse::empty())).into_response(),
    };
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let res: Vec<()> = vec![];
    let bodies: Vec<BodyRecord> =
        sqlx::query_as!(BodyRecord, r"select body from post where id=?", model.id)
            .fetch_all(&mut pool)
            .await
            .unwrap();
    let last_body = match bodies.get(0) {
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::new(res, 0))).into_response(),
        Some(b) => b.body.to_owned(),
    };
    let changes = witwiki_difffoo::get_changes(&last_body, &model.body);
    sqlx::query!(
        r"
          begin transaction;
          update post
          set
            body=?,
            title=?
          where id=?;
          commit transaction;
        ",
        model.body,
        model.title,
        model.id,
    )
    .fetch_all(&mut pool)
    .await
    .unwrap();
    Json(ApiResponse::new(res, 0)).into_response()
}
