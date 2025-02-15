use anyhow::Context;
use spin_sdk::http::{Params, Request, Response};

use crate::{
    commands::{create_todo, delete_todo, update_todo},
    models::Todo,
    queries::{find_todo, load_todos},
    request::{CreateTodo, UpdateTodo},
};

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
pub(crate) async fn list_todos_handler(_req: Request, _params: Params) -> anyhow::Result<Response> {
    let todos = load_todos().await?;
    json_response(200, &todos)
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
pub(crate) async fn create_todo_handler(req: Request, _params: Params) -> anyhow::Result<Response> {
    let new_todo: CreateTodo = serde_json::from_slice(req.body())?;
    let todo = create_todo(new_todo).await?;
    json_response(201, &todo)
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
pub(crate) async fn find_todo_handler(_req: Request, params: Params) -> anyhow::Result<Response> {
    let id: u64 = params
        .get("id")
        .unwrap_or("0")
        .parse()
        .context("Invalid id")?;
    let todo = find_todo(id).await?;
    json_response(200, &todo)
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
pub(crate) async fn update_todo_handler(req: Request, params: Params) -> anyhow::Result<Response> {
    let id: u64 = params
        .get("id")
        .unwrap_or("0")
        .parse()
        .context("Invalid id")?;
    let update: UpdateTodo = serde_json::from_slice(req.body())?;
    let todo = update_todo(id, update).await?;
    json_response(200, &todo)
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
pub(crate) async fn delete_todo_handler(_req: Request, params: Params) -> anyhow::Result<Response> {
    let id: u64 = params
        .get("id")
        .unwrap_or("0")
        .parse()
        .context("Invalid id")?;
    delete_todo(id).await?;
    Ok(Response::builder().status(204).body(String::new()).build())
}
