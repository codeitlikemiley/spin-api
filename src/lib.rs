pub(crate) mod api;
pub(crate) mod commands;
pub(crate) mod db;
pub(crate) mod handlers;
pub(crate) mod models;
pub(crate) mod queries;
pub(crate) mod request;

use handlers::open_api::openapi_handler;
use handlers::swagger::swagger_handler;

use models::Todo;
use request::{CreateTodo, UpdateTodo};
use spin_sdk::http::{Request, Response, Router};
use spin_sdk::http_component;

use crate::handlers::todos::{
    create_todo_handler, delete_todo_handler, find_todo_handler, list_todos_handler,
    update_todo_handler,
};

#[http_component]
async fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get_async("/openapi.json", openapi_handler);
    router.get_async("/todos", list_todos_handler);
    router.post_async("/todos", create_todo_handler);
    router.get_async("/todos/:id", find_todo_handler);
    router.put_async("/todos/:id", update_todo_handler);
    router.delete_async("/todos/:id", delete_todo_handler);
    router.get_async("/", swagger_handler);
    router.handle(req)
}
