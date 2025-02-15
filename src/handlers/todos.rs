use anyhow::Context;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response},
    sqlite::{Connection, Value},
};

use crate::{
    models::Todo,
    request::{CreateTodo, UpdateTodo},
};

fn get_connection() -> anyhow::Result<Connection> {
    Connection::open_default().context("Failed to open database connection")
}

fn json_response<T: serde::Serialize>(status: u16, data: &T) -> anyhow::Result<Response> {
    let body = serde_json::to_string(data)?;
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(body)
        .build())
}

#[utoipa::path(
    get,
    path = "/todos",
    tags = ["Todos"],
    responses(
        (status = 200, description = "List all todos", body = [Todo]),
        (status = 500, description = "Unexpected Error")
    )
)]
pub(crate) fn list_todos_handler(
    _req: Request,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let todos = load_todos()?;
    json_response(200, &todos)
}

fn load_todos() -> anyhow::Result<Vec<Todo>> {
    let conn = get_connection()?;
    let rows = conn.execute("SELECT id, title, completed FROM todos", &[])?;
    rows.rows().map(|row| Todo::from_row(&row)).collect()
}

#[utoipa::path(
    post,
    path = "/todos",
    tags = ["Todos"],
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created", body = Todo),
        (status = 500, description = "Unexpected Error")
    )
)]
pub(crate) fn create_todo_handler(
    req: Request,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let new_todo: CreateTodo = serde_json::from_slice(req.body())?;
    let todo = create_todo(new_todo)?;
    json_response(201, &todo)
}

fn create_todo(new: CreateTodo) -> anyhow::Result<Todo> {
    let conn = get_connection()?;
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

#[utoipa::path(
    get,
    path = "/todos/{id}",
    tags = ["Todos"],
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Unexpected Error")
    ),
    params(
        ("id" = u64, Path, description = "Todo ID")
    )
)]
pub(crate) fn find_todo_handler(
    _req: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let id: u64 = params
        .get("id")
        .unwrap_or("0")
        .parse()
        .context("Invalid id")?;
    let todo = find_todo(id)?;
    json_response(200, &todo)
}

fn find_todo(id: u64) -> anyhow::Result<Todo> {
    let conn = get_connection()?;
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

#[utoipa::path(
    put,
    path = "/todos/{id}",
    tags = ["Todos"],
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo updated", body = Todo),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Unexpected Error")
    ),
    params(
        ("id" = u64, Path, description = "Todo ID")
    )
)]
pub(crate) fn update_todo_handler(
    req: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let id: u64 = params
        .get("id")
        .unwrap_or("0")
        .parse()
        .context("Invalid id")?;
    let update: UpdateTodo = serde_json::from_slice(req.body())?;
    let todo = update_todo(id, update)?;
    json_response(200, &todo)
}

fn update_todo(id: u64, update: UpdateTodo) -> anyhow::Result<Todo> {
    let conn = get_connection()?;
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
        find_todo(id)
    } else {
        anyhow::bail!("Unexpected Error")
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    tags = ["Todos"],
    responses(
        (status = 204, description = "Todo deleted"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Unexpected Error")
    ),
    params(
        ("id" = u64, Path, description = "Todo ID")
    )
)]
pub(crate) fn delete_todo_handler(
    _req: Request,
    params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let id: u64 = params
        .get("id")
        .unwrap_or("0")
        .parse()
        .context("Invalid id")?;
    delete_todo(id)?;
    Ok(Response::builder().status(204).body(String::new()).build())
}

fn delete_todo(id: u64) -> anyhow::Result<()> {
    let conn = get_connection()?;
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
