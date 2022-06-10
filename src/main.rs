#![feature(const_trait_impl)]

use crate::db::Db;
use crate::middleware::parse_cookies::middleware as cookie_middleware;
use axum::body::Body;
use axum::middleware::from_fn;
use axum::response::Html;
use axum::Extension;
use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use middleware::app_state::RequestState;
use std::sync::Arc;
use std::{io, net::SocketAddr};
use tower::limit::ConcurrencyLimitLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod authentication;
mod components;
mod db;
mod middleware;
mod page_routes;
mod pages;
mod post;
mod preferences;
mod request;
mod user;

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "witwiki=debug,tower_http=debug".into());

    let db = Arc::new(Db::new());
    (db.migrate().await).expect("db failed to migrate");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_filter))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut app: Router<Body> = Router::new();
    app = pages::bind(app);
    app = app
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(64))
        .layer(from_fn(|req, next| cookie_middleware(req, next)))
        .layer(Extension(RequestState::new(db)))
        .fallback(get_service(ServeDir::new("./browser/static")).handle_error(handle_error));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong. :|",
    )
}

async fn echo_cookies(Extension(request_state): Extension<RequestState>) -> impl IntoResponse {
    Html::from(format!("<h1>{:?}</h1>", request_state.get_cookies()))
}
