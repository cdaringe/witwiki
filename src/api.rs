use crate::middleware::app_state::RequestState;
use crate::models::recent_tags::RecentTag;
use crate::post::Post;
use axum::extract::Path;
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
    router
        .route(
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
        .route(
            "/api/posts/:slug",
            get(
                |request_state: Extension<RequestState>, Path(slug): Path<String>| async move {
                    let conn = request_state.db.connection.lock().await;

                    let posts = conn
                        .prepare(
                            r"
            select id, user_id, body, title, created_at, slug from post
            where slug=(?)
          ",
                        )
                        .unwrap()
                        .query_map([slug], |row| {
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
                    if true {
                        Ok(Json(ApiResponse::new(posts, 10)))
                    } else {
                        Err("booo")
                    }
                },
            ),
        )
        .route(
            "/api/posts_tags/recent",
            get(|request_state: Extension<RequestState>| async move {
                let conn = request_state.db.connection.lock().await;
                let values = conn
                    .prepare(
                        r"
select count(recent_tags.tag_id) as count, t.id as id, t.tag as tag from
(
  select pt.tag_id tag_id
  from (select id from post order by id desc limit 100) as posts
  inner join post_tag  pt on pt.post_id=posts.id
) as recent_tags
inner join tag t on recent_tags.tag_id=t.id
group by tag_id
order by count desc",
                    )
                    .unwrap()
                    .query_map([], |row| {
                        Ok(RecentTag {
                            id: row.get(1)?,
                            tag: row.get(2)?,
                        })
                    })
                    .unwrap()
                    .map(|r| r.unwrap())
                    .collect::<Vec<RecentTag>>();
                if true {
                    let len = values.len();
                    println!("len: {}", len);
                    Ok(Json(ApiResponse::new(values, len)))
                } else {
                    Err("unimplemented")
                }
            }),
        )
}
