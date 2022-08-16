use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct PostComment {
    pub id: i64,
    pub salt: String,
    pub user_id: i64,
    pub hash: String,
}
