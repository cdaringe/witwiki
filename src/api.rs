use crate::middleware::app_state::RequestState;
use crate::post::Post;
use axum::{extract::Query, routing::get, Extension, Json, Router};
use rusqlite::NO_PARAMS;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ApiResponse<T>
where
    T: Serialize,
{
    values: T,
    total: usize,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(values: T, total: usize) -> Self {
        Self { values, total }
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
struct GetPostsQuery {
    limit: usize,
    offset: usize,
}

pub fn bind(router: Router) -> Router {
    router.route(
        "/api/posts/recent",
        get(
            |request_state: Extension<RequestState>, q: Query<GetPostsQuery>| async move {
                let conn = request_state.db.connection.lock().await;
                match q.limit {
                    0..=100 => {
                        let posts = conn
                            .prepare(
                                r"
              select id, user_id, substring(body, 0, 200) as body, title, created_at, slug from post
              order by id desc
              limit 10
            ",
                            )
                            .unwrap()
                            .query_map([], |row| {
                                Ok(Post {
                                    id: row.get(0)?,
                                    user_id: row.get(1)?,
                                    body: row.get(2)?,
                                    title: row.get(3)?,
                                    created_at: row.get(4)?,
                                    slug: row.get(5)?,
                                })
                            })
                            .unwrap()
                            .map(|r| r.unwrap())
                            .collect::<Vec<Post>>();
                        Ok(Json(ApiResponse::new(posts, 10)))
                    }
                    _ => Err("invalid query"),
                }
            },
        ),
    )
}
