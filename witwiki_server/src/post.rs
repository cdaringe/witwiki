use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    pub id: usize,
    pub user_id: usize,
    pub body: String,
    pub title: String,
    pub created_at: usize,
    pub updated_at: usize,
    pub slug: String,
}

impl Post {}
