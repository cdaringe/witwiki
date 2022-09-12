use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Extension, Json,
};
use comrak;
use serde::Deserialize;
use sqlx;

use crate::{models::api_response::ApiResponse, post::Post, RequestState};

#[derive(Deserialize)]
pub struct GetPostQuery {
    pub body_as: String,
}

pub async fn get(
    request_state: Extension<RequestState>,
    Path(slug): Path<String>,
    q: Query<GetPostQuery>,
) -> impl IntoResponse {
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let mut posts: Vec<Post> = sqlx::query_as!(
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
    for p in &mut posts {
        if q.body_as == "html" {
            p.body = comrak::markdown_to_html(&p.body, &comrak::ComrakOptions::default())
        }
    }
    Json(ApiResponse::new(posts, 1)).into_response()
}
