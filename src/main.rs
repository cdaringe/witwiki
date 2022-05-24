use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use std::{io, net::SocketAddr};
use tower::limit::ConcurrencyLimitLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod components;
mod pages;
mod routing;

#[tokio::main]
async fn main() {
    // migrate();
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "witwiki=debug,tower_http=debug".into());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_filter))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut app: _ = Router::new();
    app = pages::bind(app);
    let next_app = app
        .route("/foo", get(|| async { "Hi from /foo" }))
        .fallback(get_service(ServeDir::new("./browser/static")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(64));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(next_app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong. :|",
    )
}
