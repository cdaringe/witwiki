use axum::{extract::Path, response::IntoResponse, Extension, Json};
use sqlx;

use crate::{models::api_response::ApiResponse, post::Post, RequestState};

pub async fn get(
    request_state: Extension<RequestState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let posts: Vec<Post> = sqlx::query_as!(
        Post,
        r"
            select id, user_id, body, title, created_at, updated_at, slug from post
            where slug= ?
          ",
        slug
    )
    .fetch_all(&mut pool)
    .await
    .unwrap();
    Json(ApiResponse::new(posts, 1)).into_response()
}
