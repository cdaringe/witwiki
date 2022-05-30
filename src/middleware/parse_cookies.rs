use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{body::Body, http::Request};
use cookie::Cookie;
use std::collections::HashMap;

use super::app_state::RequestState;

type CookiesByName<'a> = HashMap<String, Cookie<'a>>;

pub fn get_cookies_by_name(cookies_str: &str) -> CookiesByName<'static> {
    cookies_str
        .split(";")
        .into_iter()
        .filter_map(|cookie_str| {
            println!("cookie_str: {:?}", cookie_str);
            match Cookie::parse(cookie_str) {
                Ok(cookie) => Some(cookie),
                _ => None,
            }
        })
        .fold(HashMap::new(), |mut map, cookie| {
            map.insert(cookie.name().to_owned(), cookie.into_owned());
            map
        })
}

pub async fn middleware(mut req: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    let cookies_str = req
        .headers()
        .clone()
        .get("Cookie")
        .map_or_else(|| "", |v| v.to_str().unwrap_or(""))
        .to_owned();
    let ext = req.extensions_mut().get_mut::<RequestState>().unwrap();
    ext.set_cookies(get_cookies_by_name(&cookies_str));
    next.run(req).await
}
