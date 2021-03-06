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
    pub fn new_in_mem() -> Self {
        Db {
            connection: Mutex::new(Connection::open_in_memory().unwrap()),
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
        for migration_file in migrations {
            for migration in migration_file
                .split("-- migration")
                .map(|v| v.trim())
                .filter(|v| v.len() > 0)
            {
                match conn.execute_batch(migration) {
                    Ok(_) => (),
                    Err(e) => {
                        panic!("failed to migrate: {:?}\n\nmigration:\n{}", e, migration)
                    }
                }
            }
        }
        Ok(())
    }
}
