use axum::{
    routing::{get, post},
    Router,
};

use crate::routes::{login::login, logout::logout, posts, posts_comments, posts_tags};

pub fn bind(router: Router) -> Router {
    router
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/posts/recent", get(posts::recent::get))
        .route("/api/posts/:slug", get(posts::get::get))
        // .route("/api/posts/:slug", patch(|| unimplemented!("")))
        .route("/api/posts/:slug/comments", get(posts_comments::get::get))
        .route("/api/posts_tags/recent", get(posts_tags::get::get))
}
