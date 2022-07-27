use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RecentTag {
    pub id: usize,
    pub tag: String,
}
