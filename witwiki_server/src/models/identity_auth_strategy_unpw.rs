use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct IdentityUnPw {
    pub id: i64,
    pub hash: String,
    pub user_id: i64,
    pub salt: String,
}
