use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use sqlx::{self, Acquire};

use crate::{error::e500, models::api_response::ApiResponse, RequestState};

#[derive(Deserialize)]
pub struct PatchQuery {
    id: i64,
    body: String,
    title: String,
    _change_description: Option<String>,
}

#[derive(Deserialize)]
pub struct BodyRecord {
    body: String,
}

pub async fn patch(
    Json(model): Json<PatchQuery>,
    Extension(mut request_state): Extension<RequestState>,
) -> impl IntoResponse {
    // @todo, authorize route, not authenticate route
    if !request_state.is_authenticated() {
        return (StatusCode::FORBIDDEN, Json(ApiResponse::empty())).into_response();
    }
    let pool_mutex = &request_state.db.pool;
    let res: Vec<()> = vec![];
    let bodies: Vec<BodyRecord> = sqlx::query_as!(
        BodyRecord,
        r"select body from post where id=? limit 1",
        model.id
    )
    .fetch_all(&mut pool_mutex.lock().await.acquire().await.unwrap())
    .await
    .unwrap();
    let last_body = match bodies.get(0) {
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::empty())).into_response(),
        Some(b) => b.body.to_owned(),
    };
    let _changes = witwiki_difffoo::get_changes(&last_body, &model.body);
    let mut pool = pool_mutex.lock().await.acquire().await.unwrap();
    let res = match pool.begin().await {
        Ok(mut txn) => {
            sqlx::query!(
                r"
          update post
          set
            body=?,
            title=?
          where id=?;
        ",
                model.body,
                model.title,
                model.id,
            )
            .fetch_all(&mut txn)
            .await
            .unwrap();

            match txn.commit().await {
                Ok(_) => Json(ApiResponse::new(res, 0)).into_response(),
                Err(e) => e500(&format!("{:?}", e), &"").into_response(),
            }
        }
        Err(e) => return e500(&format!("{:?}", e), &"").into_response(),
    };
    res
}
