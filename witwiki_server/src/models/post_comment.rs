use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct PostComment {
    pub id: i64,
    pub body: String,
    pub user_id: i64,
    pub created_at: i64,
}
