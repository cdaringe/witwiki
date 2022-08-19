use serde::Serialize;

#[derive(Debug, Serialize, sqlx::Type)]
pub struct RecentTag {
    pub id: i64,
    pub tag: String,
}
