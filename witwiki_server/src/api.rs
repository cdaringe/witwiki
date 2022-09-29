use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Router,
};

use crate::routes::{
    login::login,
    logout::logout,
    posts::{self},
    posts_comments, posts_tags,
};

async fn fourohfour() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "404 // did you mean /api?".to_owned(),
    )
        .into_response()
}

pub fn bind(router: Router) -> Router {
    router
        .route("/", get(fourohfour))
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/posts/recent", get(posts::recent::get))
        .route(
            "/api/posts/:slug",
            get(posts::get::get).patch(posts::patch::patch),
        )
        .route("/api/posts/:slug/comments", get(posts_comments::get::get))
        .route("/api/posts_tags/recent", get(posts_tags::get::get))
}
