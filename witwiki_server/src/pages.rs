#![allow(dead_code, unused)]

use crate::page_routes;
/**
 * @todo can we use https://github.com/launchbadge/sqlx#querying instead??
 */
use crate::{
    components::{head, page, pages},
    middleware::app_state::RequestState,
    post::Post,
};
use axum::{extract::Path, response::Html, routing::get, Extension, Router};
use witwiki_common::rusqlite;

pub fn bind(router: Router) -> Router {
    let posts_handler =
        |Path(id): Path<usize>, request_state: Extension<RequestState>| async move {};
    router
        .route(
            "/",
            get(|request_state: Extension<RequestState>| async move {
                let conn = request_state.db.connection.lock().await;
                let recent_posts = conn
                    .prepare(
                        "select id, user_id, body, title, created_at, slug from post limit :numposts",
                    )
                    .unwrap()
                    .query_and_then(&[(":numposts", "10")], |row| {
                        Ok(Post {
                            id: row.get(0)?,
                            user_id: row.get(1)?,
                            body: row.get(2)?,
                            title: row.get(3)?,
                            created_at: row.get(4)?,
                            updated_at: row.get(5)?,
                            slug: row.get(6)?
                        })
                    })
                    .expect("posts query failed")
                    .map(|x: Result<Post, rusqlite::Error>| x.unwrap())
                    .collect::<Vec<Post>>();
                Html(page::page(
                    &request_state,
                    &head::head(&"home", None),
                    vec![],
                    &&pages::home::home(&"", recent_posts, &"").await,
                ))
            }),
        )
        .route("/wiki/:id", get(page_routes::wiki::get::handle_get))
        .route(
            "/wiki/edit/:id",
            get(page_routes::wiki::edit::get::handle_get),
        )
        .route(
            "/x/user/settings",
            get(|request_state: Extension<RequestState>| async move {
                Html(page::page(
                    &request_state,
                    &head::head(&"user settings", None),
                    vec![],
                    &&pages::user_settings::user_settings(&"", &""),
                ))
            }),
        )
}
