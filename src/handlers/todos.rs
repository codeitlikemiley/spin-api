use anyhow::Context;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response},
    key_value::Store,
};

use crate::{
    models::Todo,
    request::{CreateTodo, UpdateTodo},
};

#[utoipa::path(
    get,
    path = "/todos",
    tags = ["Todos"],
    responses(
        (status = 200, description = "List all todos", body = [Todo])
    )
)]
pub(crate) fn list_todos_handler(
    _req: Request,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let todos = load_todos()?;
    let body = serde_json::to_string(&todos)?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(body)
        .build())
}

#[utoipa::path(
    post,
    path = "/todos",
    tags = ["Todos"],
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created", body = Todo)
    )
)]
pub(crate) fn create_todo_handler(
    req: Request,
    _params: Params,
) -> anyhow::Result<impl IntoResponse> {
    let new_todo: CreateTodo = serde_json::from_slice(req.body())?;
    let todo = create_todo(new_todo)?;
    let body = serde_json::to_string(&todo)?;
    Ok(Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(body)
        .build())
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    tags = ["Todos"],
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = 404, description = "Todo not found")
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
        .context("invalid id")?;
    if let Some(todo) = find_todo(id)? {
        let body = serde_json::to_string(&todo)?;
        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(body)
            .build())
    } else {
        Ok(Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body("Todo not found")
            .build())
    }
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    tags = ["Todos"],
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo updated", body = Todo),
        (status = 404, description = "Todo not found")
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
        .context("invalid id")?;
    let update: UpdateTodo = serde_json::from_slice(req.body())?;
    if let Some(todo) = update_todo(id, update)? {
        let body = serde_json::to_string(&todo)?;
        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(body)
            .build())
    } else {
        Ok(Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body("Todo not found")
            .build())
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    tags = ["Todos"],
    responses(
        (status = 204, description = "Todo deleted")
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
        .context("invalid id")?;
    if delete_todo(id)? {
        Ok(Response::builder()
            .status(204)
            .header("Access-Control-Allow-Origin", "*")
            .body("")
            .build())
    } else {
        Ok(Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body("Todo not found")
            .build())
    }
}

fn load_todos() -> anyhow::Result<Vec<Todo>> {
    let store = Store::open_default()?;
    if let Some(todos) = store.get_json("todos")? {
        Ok(todos)
    } else {
        Ok(vec![])
    }
}

fn save_todos(todos: &[Todo]) -> anyhow::Result<()> {
    let store = Store::open_default()?;
    store.set_json("todos", &Vec::from(todos))?;
    Ok(())
}

fn create_todo(new: CreateTodo) -> anyhow::Result<Todo> {
    let mut todos = load_todos()?;
    let new_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let todo = Todo {
        id: new_id,
        title: new.title,
        completed: false,
    };
    todos.push(todo.clone());
    save_todos(&todos)?;
    Ok(todo)
}

fn find_todo(id: u64) -> anyhow::Result<Option<Todo>> {
    let todos = load_todos()?;
    Ok(todos.into_iter().find(|t| t.id == id))
}

fn update_todo(id: u64, update: UpdateTodo) -> anyhow::Result<Option<Todo>> {
    let mut todos = load_todos()?;
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if let Some(title) = update.title {
            todo.title = title;
        }
        if let Some(completed) = update.completed {
            todo.completed = completed;
        }
        let updated = todo.clone();
        save_todos(&todos)?;
        Ok(Some(updated))
    } else {
        Ok(None)
    }
}

fn delete_todo(id: u64) -> anyhow::Result<bool> {
    let mut todos = load_todos()?;
    let original_len = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() < original_len {
        save_todos(&todos)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
