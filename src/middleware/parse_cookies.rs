use axum::extract::{FromRequest, RequestParts};
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::Extension;
use axum::{body::Body, http::Request};
use cookie::Cookie;
use std::borrow::BorrowMut;
use std::collections::HashMap;

use super::app_state::RequestState;

type CookiesByName<'a> = HashMap<String, Cookie<'a>>;

pub fn get_cookies_by_name(headers: &HeaderMap) -> CookiesByName<'static> {
    headers
        .get("Cookie")
        .map_or_else(|| "", |v| v.to_str().unwrap_or(""))
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

pub async fn middleware(req: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    let mut request_parts = RequestParts::new(req);
    let Extension(mut request_state) =
        Extension::<RequestState>::from_request(request_parts.borrow_mut())
            .await
            .expect("ext missing");
    let mut next_request = request_parts.try_into_request().unwrap();
    let headers = next_request.headers().clone();
    request_state.cookies_by_name = get_cookies_by_name(&headers);
    next_request
        .borrow_mut()
        .extensions_mut()
        .insert(request_state);
    if false {
        Err("")
    } else {
        Ok(next.run(next_request).await)
    }
}
