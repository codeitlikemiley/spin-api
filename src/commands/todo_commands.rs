use anyhow::Context;
use spin_sdk::sqlite::Value;

use crate::{db::db_connection, queries::find_todo, CreateTodo, Todo, UpdateTodo};

pub(crate) async fn create_todo(new: CreateTodo) -> anyhow::Result<Todo> {
    let conn = db_connection().await?;
    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?, false)",
        &[Value::Text(new.title.clone())],
    )?;
    let id = conn
        .execute("SELECT last_insert_rowid() AS id", &[])?
        .rows()
        .next()
        .and_then(|row| row.get("id"))
        .context("Failed to retrieve last inserted ID")?;
    Ok(Todo {
        id,
        title: new.title,
        completed: false,
    })
}

pub(crate) async fn update_todo(id: u64, update: UpdateTodo) -> anyhow::Result<Todo> {
    let conn = db_connection().await?;
    let mut query = "UPDATE todos SET".to_string();
    let mut args = Vec::new();

    if let Some(title) = update.title {
        query.push_str(" title = ?,");
        args.push(Value::Text(title));
    }
    if let Some(completed) = update.completed {
        query.push_str(" completed = ?,");
        args.push(Value::Integer(completed as i64));
    }

    query.pop();
    query.push_str(" WHERE id = ?");
    args.push(Value::Integer(id as i64));

    let affected = conn.execute(&query, &args)?.rows.len();
    if affected > 0 {
        find_todo(id).await
    } else {
        anyhow::bail!("Unexpected Error")
    }
}

pub(crate) async fn delete_todo(id: u64) -> anyhow::Result<()> {
    let conn = db_connection().await?;
    let affected = conn
        .execute(
            "DELETE FROM todos WHERE id = ?",
            &[Value::Integer(id as i64)],
        )?
        .rows
        .len();
    if affected > 0 {
        Ok(())
    } else {
        anyhow::bail!("Unexpected Error")
    }
}
