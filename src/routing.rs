use axum::http::{HeaderMap, HeaderValue, Request};
// use axum::{extract::Extension, routing::get, Router};
use axum::middleware::Next;
use cookie::Cookie;
use std::collections::HashMap;
use std::sync::Arc;

type ParsedCookieLayer<'a> = HashMap<String, Cookie<'a>>;

pub async fn create_state<B: 'static>(
    req: Request<B>,
    next: Next<B>,
) -> Arc<ParsedCookieLayer<'static>> {
    let cookies_by_name = req
        .headers()
        .get("cookie")
        .map_or_else(|| "", |v| v.to_str().unwrap_or(""))
        .split(";")
        .into_iter()
        .filter_map(|cookie_str| match Cookie::parse(cookie_str) {
            Ok(cookie) => Some(cookie),
            _ => None,
        })
        .fold(HashMap::new(), |mut map, cookie| {
            map.insert(cookie.name().to_owned(), cookie.to_owned());
            map
        });
    Arc::new(cookies_by_name)
}
