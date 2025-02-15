use anyhow::Result;
use spin_sdk::sqlite::Value;

use crate::{db::db_connection, Todo};

pub(crate) async fn load_todos() -> Result<Vec<Todo>> {
    let conn = db_connection().await?;
    let query_result = conn.execute("SELECT id, title, completed FROM todos", &[])?;
    query_result
        .rows()
        .map(|row| Todo::from_row(&row))
        .collect()
}

pub(crate) async fn find_todo(id: u64) -> anyhow::Result<Todo> {
    let conn = db_connection().await?;

    conn.execute(
        "SELECT id, title, completed FROM todos WHERE id = ?",
        &[Value::Integer(id as i64)],
    )?
    .rows
    .into_iter()
    .find_map(|row| {
        Some(Todo {
            id: row.get::<u64>(0)?,
            title: row.get::<&str>(1)?.to_string(),
            completed: row.get::<bool>(2)?,
        })
    })
    .ok_or_else(|| anyhow::anyhow!("Todo not found"))
}
