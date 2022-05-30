use crate::{
    components::{head, nav, page, pages},
    middleware::app_state::RequestState,
};
use axum::{response::Html, routing::get, Extension, Router};

pub fn bind(router: Router) -> Router {
    router.route(
        "/",
        get(|request_state: Extension<RequestState>| async move {
            let is_auth = request_state.get_cookies().get("foo").is_some();
            let auth_link = if is_auth {
                nav::link("/logout", "logout")
            } else {
                nav::link("/login", "login")
            };
            let mut action_links = if is_auth {
                vec![
                    nav::link("/settings", "settings"),
                    nav::link("/edit", "edit"),
                    nav::link("/add", "add"),
                ]
            } else {
                vec![]
            };
            action_links.insert(0, auth_link);
            let html = page::page(
                &head::head(
                    &"home",
                    Some(
                        &(vec![
                            "<link rel='stylesheet' href='css/water.css'>",
                            "<link rel='stylesheet' href='css/page.css'>",
                            "<link rel='stylesheet' href='css/utility.css'>",
                        ])
                        .concat(),
                    ),
                ),
                &nav::header(
                    &vec![nav::link("/", "home"), nav::link("/browse", "browse")],
                    &action_links,
                    "",
                ),
                &pages::home::home(&"", &""),
            );
            Html(html)
        }),
    )
}
