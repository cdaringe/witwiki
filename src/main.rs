#![feature(const_trait_impl)]

use crate::middleware::parse_cookies::middleware as cookie_middleware;
use axum::body::{Body, Bytes};
use axum::extract::Path;
use axum::middleware::from_fn;
use axum::Extension;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use middleware::app_state::RequestState;
use std::sync::Arc;
use std::{io, net::SocketAddr};
use tower::limit::ConcurrencyLimitLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod components;
mod middleware;
mod pages;

#[tokio::main]
async fn main() {
    // migrate();
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "witwiki=debug,tower_http=debug".into());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_filter))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut app: Router<Body> = Router::new();
    app = pages::bind(app);
    app = app
        .route("/foo", get(foo))
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(64))
        .layer(from_fn(|req, next| cookie_middleware(req, next)))
        .layer(Extension(RequestState::default()))
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

async fn foo(Extension(request_state): Extension<RequestState>) -> Result<Bytes, StatusCode> {
    println!(
        "num cookies in request_state: {:?}",
        request_state.cookies_by_name.len()
    );
    if true {
        Ok(Bytes::from("abc"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
