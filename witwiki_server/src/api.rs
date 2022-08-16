#![allow(dead_code, unused)]

use std::collections::{hash_set, HashSet};
use std::io::Error;

use crate::authentication::{self, Authenticated};
use crate::middleware::app_state::RequestState;
use crate::models::identity_auth_strategy_unpw::IdentityUnPw;
use crate::models::jwt::{encode, Claims};
use crate::models::post_comment::PostComment;
use crate::models::recent_tags::RecentTag;
use crate::models::user::User;
use crate::post::Post;
use axum::body::Body;
use axum::extract::Path;
use axum::http::header::SET_COOKIE;
use axum::http::{Request, StatusCode};
use axum::response::{AppendHeaders, IntoResponse};
use axum::{
    extract::{Json as ExtractJson, Query},
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{offset, DateTime, Utc};
use cookie::time::{Duration, OffsetDateTime};
use cookie::Cookie;
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

async fn login(
    request_state: Extension<RequestState>,
    // req: Request<Body>,
    body: Json<AuthenticationUnPwBody>,
) -> impl IntoResponse {
    // let authority = match req.uri().authority() {
    //     Some(v) => v.to_string(),
    //     None => return (StatusCode::BAD_REQUEST).into_response(),
    // };
    let authority = "localhost:9999".to_string();
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let user_result: Result<User, _> = sqlx::query_as!(
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
    if let Ok(user) = user_result
        && user.authentication_strategy == 1
        && let Ok(identity) = sqlx::query_as!(
                IdentityUnPw,
                r"
          select
            id,
            hash,
            user_id,
            salt
          from identity_authentication_strategy_unpw
          where user_id = ?
          ",
                user.id
            )
            .fetch_one(&mut pool)
            .await
        {
            match authentication::authenticate(&body.password, &identity.hash) {
              Ok(auth_state) => {
                if auth_state == Authenticated::In {
                  let duration = Duration::days(1);
                  let s = duration.as_seconds_f64() as usize;
                  let session_jwt = encode(&Claims {
                    sub: "wee".to_string(),
                    exp: s,
                    roles: HashSet::new()
                  }, "@todo").unwrap();
                  let jwt_cookie = Cookie::build("jwt", session_jwt).domain(authority).path("/").secure(true).http_only(true).max_age(duration).finish();
                  return (StatusCode::OK,
                    AppendHeaders([(SET_COOKIE, format!("{key}={value}", key=jwt_cookie.name(), value=jwt_cookie.value()))]),
                    Json(ApiResponse::new([true], 10))
                  ).into_response();
                }
              },
              Err(v) => {
                println!("unexpected authorization failure: {}.\nare DB records invalid?", v);
                return (StatusCode::INTERNAL_SERVER_ERROR, Err::<(), &str>("500")).into_response()
              }
            }

    }
    (StatusCode::UNAUTHORIZED, Err::<(), &str>("409")).into_response()
}

pub fn bind(router: Router) -> Router {
    router
    .route(
      /**
curl -X POST -H "Content-Type: application/json" \
  -d '{"username": "raptorboy", "password": "password"}' \
  http://localhost:9999/api/login
       */
      "/api/login",
      post(login),
  )
        .route(
            "/api/posts/recent",
            get(     |request_state: Extension<RequestState>, q: Query<GetPostsQuery>| async move {
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
