use crate::{
    models::{api_response::ApiResponse, recent_tags::RecentTag},
    RequestState,
};
use axum::{response::IntoResponse, Extension, Json};
use sqlx;

pub async fn get(request_state: Extension<RequestState>) -> impl IntoResponse {
    let mut pool = request_state.db.pool.lock().await.acquire().await.unwrap();
    let tags: Vec<RecentTag> = sqlx::query_as!(
        RecentTag,
        r#"
select t.id as "id!", t.tag as "tag!" from
(
  select pt.tag_id tag_id
  from (select id from post order by id desc limit 100) as posts
  inner join post_tag pt on pt.post_id=posts.id
) as recent_tags
inner join tag t on recent_tags.tag_id=t.id
group by tag_id
order by count(recent_tags.tag_id) desc"#,
    )
    .fetch_all(&mut pool)
    .await
    .unwrap();
    let total = tags.len();
    Json(ApiResponse::new(tags, total)).into_response()
}
