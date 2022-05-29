use crate::components::{head, nav, page};
use axum::{http::header::HeaderMap, response::Html, routing::get, Router};

// static LINKS: Link
pub fn bind(router: Router) -> Router {
    router.route(
        "/",
        get(|headers: HeaderMap| async move {
            let auth_link = match headers.get("cookie") {
                Some(_) => nav::link("/logout", "logout"),
                None => nav::link("/login", "login"),
            };
            let html = page::page(
                &head::head(&"home", None),
                &nav::nav(
                    &vec![
                        nav::link("/", "home"),
                        nav::link("/browse", "browse"),
                        nav::link("/edit", "edit"),
                        auth_link,
                    ],
                    "",
                ),
                &"",
            );
            Html(html)
        }),
    )
    // .layer(from_fn(create_state))
}
