use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use comrak;

use serde::Deserialize;
use sqlx::{self, Acquire};

use crate::{
    error::{e400, e500},
    models::api_response::ApiResponse,
    RequestState,
};

#[derive(Deserialize)]
pub struct PatchQuery {
    body: String,
    title: String,
    _change_description: Option<String>,
}

#[derive(Deserialize)]
pub struct BodyRecord {
    body: String,
}

pub async fn patch(
    Path(id): Path<i64>,
    Json(model): Json<PatchQuery>,
    Extension(mut request_state): Extension<RequestState>,
) -> impl IntoResponse {
    // @todo, authorize route, not authenticate route
    if !request_state.is_authenticated() {
        return (StatusCode::FORBIDDEN, Json(ApiResponse::empty())).into_response();
    }
    let pool_mutex = &request_state.db.pool;
    let res: Vec<()> = vec![];
    let bodies: Vec<BodyRecord> =
        sqlx::query_as!(BodyRecord, r"select body from post where id=? limit 1", id)
            .fetch_all(&mut pool_mutex.lock().await.acquire().await.unwrap())
            .await
            .unwrap();
    let last_body = match bodies.get(0) {
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::empty())).into_response(),
        Some(b) => b.body.to_owned(),
    };
    let _changes = witwiki_difffoo::get_changes(&last_body, &model.body);
    let mut pool = pool_mutex.lock().await.acquire().await.unwrap();
    if model.body.len() == 0 {
        return e400("missing body", "missing body").into_response();
    }
    let res = match pool.begin().await {
        Ok(mut txn) => {
            let md_body = html2md::parse_html(&model.body);
            sqlx::query!(
                r"
          update post
          set
            body=?,
            title=?
          where id=?;
        ",
                md_body,
                model.title,
                id,
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
