#![allow(dead_code, unused)]

use std::io::Error;

use crate::middleware::app_state::RequestState;
use crate::models::post_comment::PostComment;
use crate::models::recent_tags::RecentTag;
use crate::models::user::User;
use crate::post::Post;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    extract::{Json as ExtractJson, Query},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx;

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

#[derive(Deserialize)]
struct AuthenticationUnPwBody {
    password: String,
    username: String,
}

// async fn please_login(auth: Json<AuthenticationUnPwBody>) -> Result<String, Error> {
//     Ok(String::from("weee"))
// }
async fn login(
    request_state: Extension<RequestState>,
    body: Json<AuthenticationUnPwBody>,
) -> impl IntoResponse {
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let user_opt: Result<User, _> = sqlx::query_as!(
        User,
        r"
select
id,
username,
first_name,
last_name,
user_preferences_id,
authentication_strategy
from user
where username = ?
",
        body.username
    )
    .fetch_one(&mut pool)
    .await;
    if user_opt.is_err() {
        return (StatusCode::UNAUTHORIZED, Err("bummer"));
    }
    if user_opt.unwrap().authentication_strategy != 0 {
        return (StatusCode::BAD_GATEWAY, Err("unimplemented"));
    }

    if true {
        (StatusCode::OK, Ok(Json(ApiResponse::new([true], 10))))
    } else {
        (StatusCode::BAD_GATEWAY, Err("invalid query"))
    }
}

pub fn bind(router: Router) -> Router {
    router
    .route(
      "/api/login",
      post(login),
  )
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
                    let comments: Vec<PostComment> = sqlx::query_as!(PostComment,
                        r"
select
  pc.id,
  pc.body,
  pc.user_id,
  pc.created_at
from post_comment pc
inner join post p on p.id=pc.post_id
where p.slug = ?
limit 1000
",
                        slug
                    )
                              .fetch_all(&mut pool)
                    .await
                    .unwrap();
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
