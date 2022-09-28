use crate::models::identity_auth_strategy_unpw::IdentityUnPw;
use sqlx::{pool::PoolConnection, Sqlite};

pub async fn query_unpw_identity(
    user_id: i64,
    pool: &mut PoolConnection<Sqlite>,
) -> Result<IdentityUnPw, sqlx::Error> {
    sqlx::query_as!(
        IdentityUnPw,
        r"
select
id,
hash,
user_id,
salt
from identity_authentication_strategy_unpw
where user_id = ?
",
        user_id
    )
    .fetch_one(pool)
    .await
}
