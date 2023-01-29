#![allow(warnings)]

use std::{io, net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get_service,
    Extension, Router,
};
use models::api_response::ApiResponse;
use tower::limit::ConcurrencyLimitLayer;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use middleware::app_state::RequestState;
use witwiki_common::{dotenv::dotenv, tokio};
use witwiki_db::Db;

mod api;
mod authentication;
mod config;
mod dao;
mod db;
mod error;
mod middleware;
mod models;
mod post;
mod preferences;
mod request;
mod routes;

#[tokio::main]
async fn main() -> Result<(), String> {
    let config = crate::config::get_config();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let db = Arc::new(Db::new().await?);
    let mut app: Router<Body> = Router::new();
    app = api::bind(app);

    if !config.is_cors_enabled {
        app = app.layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::CONNECT,
                    Method::DELETE,
                    Method::GET,
                    Method::HEAD,
                    Method::PATCH,
                    Method::POST,
                    Method::PUT,
                ]),
        )
    }
    app = app
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(64))
        .layer(Extension(RequestState::new(db)))
        .fallback(get_service(ServeDir::new("./browser/static")).handle_error(handle_error));

    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port as u16));
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong. :|",
    )
}
