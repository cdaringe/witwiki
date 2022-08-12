#![allow(dead_code, unused)]

use serde::Serialize;

use crate::preferences::UserPreferences;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_preferences_id: Option<i64>,
    pub authentication_strategy: i64,
}
