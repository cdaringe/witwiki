use std::env::current_dir;
use witwiki_common::{
    rusqlite::Connection,
    sqlx::sqlite::{SqlitePool, SqlitePoolOptions},
    tokio::sync::Mutex,
};

#[derive(Debug)]
pub struct Db {
    pub pool: Mutex<SqlitePool>,
    pub connection: Mutex<Connection>,
}

impl Db {
    pub async fn new() -> Result<Db, std::string::String> {
        let db_filename_buf = current_dir().unwrap().join("wit.db");
        let db_filename = db_filename_buf.to_str().ok_or("invalid db path")?;
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&format!("sqlite://{}?mode=rwc", db_filename))
            .await
            .map_err(|err| format!("{}, {}", err.to_string(), db_filename))?;
        Ok(Db {
            pool: Mutex::new(pool),
            // @deprecated
            connection: Mutex::new(Connection::open(db_filename).expect("failed to open db")),
        })
    }
}
