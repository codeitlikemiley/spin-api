use anyhow::{Context, Result};
use spin_sdk::sqlite::Value;

use crate::{db::db_connection, queries::find_todo, CreateTodo, Todo, UpdateTodo};

pub(crate) async fn create_todo(new: CreateTodo) -> Result<Todo> {
    let conn = db_connection().await?;

    conn.execute("BEGIN TRANSACTION", &[])?;

    let result = conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?, false)",
        &[Value::Text(new.title.clone())],
    );

    match result {
        Ok(_) => {
            let id = conn
                .execute("SELECT last_insert_rowid() AS id", &[])?
                .rows()
                .next()
                .and_then(|row| row.get("id"))
                .context("Failed to retrieve last inserted ID")?;

            conn.execute("COMMIT", &[])?;

            Ok(Todo {
                id,
                title: new.title,
                completed: false,
            })
        }
        Err(e) => {
            conn.execute("ROLLBACK", &[])?;
            Err(e.into())
        }
    }
}

pub(crate) async fn update_todo(id: u64, update: UpdateTodo) -> Result<Todo> {
    let conn = db_connection().await?;

    conn.execute("BEGIN TRANSACTION", &[])?;

    let fields: [(&str, Option<Value>); 2] = [
        ("title", update.title.map(Value::Text)),
        (
            "completed",
            update.completed.map(|c| Value::Integer(c as i64)),
        ),
    ];

    let updates: Vec<String> = fields
        .iter()
        .filter_map(|(field, value)| value.as_ref().map(|_| format!("{} = ?", field)))
        .collect();

    let mut args: Vec<Value> = fields
        .iter()
        .filter_map(|(_, value)| value.clone())
        .collect();

    if updates.is_empty() {
        conn.execute("ROLLBACK", &[])?;
        anyhow::bail!("No fields to update");
    }

    let query = format!("UPDATE todos SET {} WHERE id = ?", updates.join(", "));
    args.push(Value::Integer(id as i64));

    let result = conn.execute(&query, &args);

    match result {
        Ok(_) => {
            conn.execute("COMMIT", &[])?;
            find_todo(id).await
        }
        Err(e) => {
            conn.execute("ROLLBACK", &[])?;
            Err(e.into())
        }
    }
}

pub(crate) async fn delete_todo(id: u64) -> Result<()> {
    let conn = db_connection().await?;
    conn.execute(
        "DELETE FROM todos WHERE id = ?",
        &[Value::Integer(id as i64)],
    )?;
    Ok(())
}
