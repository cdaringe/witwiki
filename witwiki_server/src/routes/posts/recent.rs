use crate::{models::api_response::ApiResponse, post::Post, RequestState};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use sqlx;

#[derive(Deserialize)]
pub struct GetPostsQuery {
    pub limit: i64,
    pub offset: i64,
}

pub async fn get(
    request_state: Extension<RequestState>,
    q: Query<GetPostsQuery>,
) -> impl IntoResponse {
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    match q.limit {
        0..=100 => {
            let posts: Vec<Post> = sqlx::query_as!(
                Post,
                r"
              select id, user_id, ifnull(substring(body, 0, 200), '') as body, title, created_at, updated_at, slug from post
              order by id desc
              limit ?
            ",
                q.limit
            )
            .fetch_all(&mut pool)
            .await
            .unwrap();
            Json(ApiResponse::new(posts, 10)).into_response()
        }
        _ => (StatusCode::BAD_REQUEST, "invalid limit").into_response(),
    }
}
