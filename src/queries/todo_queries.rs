use spin_sdk::sqlite::Value;

use crate::{db::db_connection, Todo};

pub(crate) async fn load_todos() -> anyhow::Result<Vec<Todo>> {
    let conn = db_connection().await?;
    let rows = conn.execute("SELECT id, title, completed FROM todos", &[])?;
    rows.rows().map(|row| Todo::from_row(&row)).collect()
}

pub(crate) async fn find_todo(id: u64) -> anyhow::Result<Todo> {
    let conn = db_connection().await?;
    let rowset = conn
        .execute(
            "SELECT id, title, completed FROM todos WHERE id = ?",
            &[Value::Integer(id as i64)],
        )?
        .rows;
    if let Some(row) = rowset.first() {
        let todo = Todo {
            id: row.get::<u64>(0).unwrap(),
            title: row.get::<&str>(1).unwrap().to_string(),
            completed: row.get(2).unwrap(),
        };
        Ok(todo)
    } else {
        anyhow::bail!("Unexpected Error")
    }
}
