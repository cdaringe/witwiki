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

pub fn bind(router: Router) -> Router {
    let posts_handler =
        |Path(id): Path<usize>, request_state: Extension<RequestState>| async move {};
    router
        .route(
            "/",
            get(|request_state: Extension<RequestState>| async move {
                let conn = request_state.db.connection.lock().await;
                let recent_posts = conn
                    .prepare("select id, user_id, body, title from post limit :numposts")
                    .unwrap()
                    .query_and_then(&[(":numposts", "10")], |row| {
                        // let ts_str = row.get::<usize, String>(4)?;
                        // println!("{:?}", ts_str);
                        Ok(Post {
                            id: row.get(0)?,
                            user_id: row.get(1)?,
                            body: row.get(2)?,
                            title: row.get(3)?,
                            // timestamp_int: DateTime::from_str(&ts_str)
                            //     .expect("failed to extract timestamp"),
                        })
                    })
                    .expect("posts query failed")
                    .map(|x: Result<Post, rusqlite::Error>| x.unwrap())
                    .collect::<Vec<Post>>();
                Html(page::page(
                    &request_state,
                    &head::head(&"home", None),
                    &&pages::home::home(&"", recent_posts, &"").await,
                ))
            }),
        )
        .route("/wiki/:id", get(posts_handler))
        .route(
            // @todo make this a */edit path wildcard
            "/wiki/edit/:id",
            get(
                |Path(id): Path<usize>, request_state: Extension<RequestState>| async move {
                    page_routes::posts::get::get(
                        &request_state,
                        page_routes::posts::common::get_page_post(&request_state.db, id).await,
                        id,
                    )
                },
            ),
        )
        .route(
            "/x/user/settings",
            get(|request_state: Extension<RequestState>| async move {
                Html(page::page(
                    &request_state,
                    &head::head(&"user settings", None),
                    &&pages::user_settings::user_settings(&"", &""),
                ))
            }),
        )
}
