use rusqlite::{Connection, Result};
use std::env::current_dir;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Db {
    pub connection: Mutex<Connection>,
}

impl Db {
    pub fn new() -> Self {
        let db_filename = current_dir().unwrap().join("wit.db");
        Db {
            connection: Mutex::new(
                Connection::open(db_filename.as_path().to_owned()).expect("failed to open db"),
            ),
        }
    }
    pub async fn migrate(&self) -> Result<()> {
        let mut migrations = vec![include_str!("../db/migrations/0001.sql")];
        migrations = [
            migrations,
            vec![include_str!("../db/migrations/dev/0001.sql")],
        ]
        .concat();
        let conn = self.connection.lock().await;
        for migration in migrations {
            conn.execute_batch(migration).expect("migration failed");
        }
        Ok(())
    }
}
