use witwiki_common::{sqlx::Executor, tokio};
use witwiki_db::Db;

#[tokio::main]
async fn main() -> Result<(), String> {
    let db = Db::new().await?;
    let mut migrations = vec![include_str!("../db/migrations/0001.sql")];
    migrations = [
        migrations,
        vec![include_str!("../db/migrations/dev/0001.sql")],
    ]
    .concat();
    let pool = db.pool.lock().await;
    for migration_file in migrations {
        for migration in migration_file
            .split("-- migration")
            .map(|v| v.trim())
            .filter(|v| v.len() > 0)
        {
            match pool.execute(migration).await {
                Ok(_) => (),
                Err(e) => {
                    panic!("failed to migrate: {:?}\n\nmigration:\n{}", e, migration)
                }
            }
        }
    }
    Ok(())
}
