use crate::{db::Db, post::Post};

pub async fn get_page_post(db: &Db, id: usize) -> Post {
    let conn = db.connection.lock().await;
    conn.query_row(
        "select id, user_id, body, title from post where id = :id",
        &[(":id", &format!("{}", id))],
        |row| {
            Ok(Post {
                id: row.get(0)?,
                user_id: row.get(1)?,
                body: row.get(2)?,
                title: row.get(3)?,
            })
        },
    )
    .unwrap()
}
