use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub body: String,
    pub title: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub slug: String,
}

impl Post {}
