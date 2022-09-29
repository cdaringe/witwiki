use crate::models::user::User;
use sqlx::{pool::PoolConnection, Sqlite};

pub struct UserQuery {
    id: Option<i64>,
    username: Option<String>,
}

impl UserQuery {
    pub fn from_username(un: &str) -> Self {
        Self {
            id: None,
            username: Some(un.to_owned()),
        }
    }
}

pub async fn get_user(
    query: UserQuery,
    pool: &mut PoolConnection<Sqlite>,
) -> Result<User, sqlx::Error> {
    let un = query.username.expect("id not supported");
    sqlx::query_as!(
        User,
        r"
select
id,
username,
first_name,
last_name,
user_preferences_id,
authentication_strategy
from user
where username = ?
",
        un
    )
    .fetch_one(pool)
    .await
}
