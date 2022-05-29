use crate::middleware::app_state;
use crate::middleware::app_state::State;
use axum::body::Body;
use axum::http::{HeaderMap};
use axum::middleware::Next;
use axum::response::IntoResponse;
use cookie::Cookie;
use std::collections::HashMap;
use std::sync::Arc;

type CookiesByName<'a> = HashMap<String, Cookie<'a>>;

pub fn get_cookies_by_name(headers: &HeaderMap) -> CookiesByName<'static> {
    headers
        .get("cookie")
        .map_or_else(|| "", |v| v.to_str().unwrap_or(""))
        .split(";")
        .into_iter()
        .filter_map(|cookie_str| match Cookie::parse(cookie_str) {
            Ok(cookie) => Some(cookie),
            _ => None,
        })
        .fold(HashMap::new(), |mut map, cookie| {
            // map.insert(cookie.name().to_owned(), cookie.to_owned());
            map.insert(cookie.name().to_owned(), cookie.into_owned());
            map
        })
}

pub async fn middleware(
    req: app_state::Req,
    next: Next<Body>,
    state_: &Arc<State>,
) -> impl IntoResponse {
    {
      let state = state_.clone();
      let mut req_state = state.get_request_state(&req);
      req_state.cookies_by_name = get_cookies_by_name(&req.headers());
      state.set_request_state(&req, req_state);
    }
    if false {
        Err("bummer")
    } else {
        Ok(next.run(req).await)
    }
}
