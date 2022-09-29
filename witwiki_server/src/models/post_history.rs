use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct PostHistory {
    pub id: i64,
    pub user_id: i64,
    pub post_id: i64,
    pub body_revision: i64,
    pub title_revision: i64,
}
