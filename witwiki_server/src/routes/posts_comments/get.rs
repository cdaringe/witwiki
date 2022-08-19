use axum::{extract::Path, response::IntoResponse, Extension, Json};
use sqlx;

use crate::{
    models::{api_response::ApiResponse, post_comment::PostComment},
    RequestState,
};

pub async fn get(
    request_state: Extension<RequestState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let comments: Vec<PostComment> = sqlx::query_as!(
        PostComment,
        r"
select
  pc.id,
  pc.body,
  pc.user_id,
  pc.created_at
from post_comment pc
inner join post p on p.id=pc.post_id
where p.slug = ?
limit 1000
",
        slug
    )
    .fetch_all(&mut pool)
    .await
    .unwrap();
    let total = comments.len();
    Json(ApiResponse::new(comments, total)).into_response()
}
