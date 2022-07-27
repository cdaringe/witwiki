use crate::middleware::app_state::RequestState;
use crate::models::post_comment::PostComment;
use crate::models::recent_tags::RecentTag;
use crate::post::Post;
use axum::extract::Path;
use axum::{extract::Query, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx;
// witwiki_db

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
              select id, user_id, substring(body, 0, 200) as body, title, created_at, updated_at, slug from post
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
                                        updated_at: row.get(5)?,
                                        slug: row.get(6)?,
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
            select id, user_id, body, title, created_at, updated_at, slug from post
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
                                updated_at: row.get(5)?,
                                slug: row.get(6)?,
                            })
                        })
                        .unwrap()
                        .map(|r| r.unwrap())
                        .collect::<Vec<Post>>();
                    if true {
                        Ok(Json(ApiResponse::new(posts, 1)))
                    } else {
                        Err("unimplemented")
                    }
                },
            ),
        )
        .route(
            "/api/posts/:slug/comments",
            get(
                |request_state: Extension<RequestState>, Path(slug): Path<String>| async move {
                    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
                    let query_res = sqlx::query!(
                        r"
select pc.* from post_comment pc
inner join post p on p.id=pc.post_id
where p.slug = ?
limit 1000
",
                        slug
                    )
                    .fetch_all(&mut pool)
                    .await
                    .unwrap();
                    let comments = query_res
                        .into_iter()
                        .map(|v| PostComment {
                            id: v.id,
                            user_id: v.user_id,
                            body: v.body,
                            created_at: v.created_at,
                        })
                        .collect::<Vec<PostComment>>();
                      if true {
                        Ok(Json(ApiResponse::new(comments, 1)))
                      } else {
                        Err("unimplemented! i cannot figure out how to map the result from the sql call above into Err(&str) gracefully, so i force compiler inference by this garbagio")
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
  inner join post_tag pt on pt.post_id=posts.id
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
                    Err(String::from("unimplemented"))
                }
            }),
        )
}