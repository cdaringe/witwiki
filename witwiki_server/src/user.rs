use crate::preferences::UserPreferences;

#[derive(Debug)]
pub struct User {
    id: isize,
    username: String,
    first_name: Option<String>,
    last_name: Option<String>,
    user_preferences_id: Option<isize>,
    user_preferences: Option<UserPreferences>,
}
