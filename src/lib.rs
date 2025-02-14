pub mod api;
pub mod handlers;
pub mod models;
pub mod request;

use handlers::llm::llm_handler;
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
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/openapi.json", openapi_handler);
    router.get("/todos", list_todos_handler);
    router.post("/todos", create_todo_handler);
    router.get("/todos/:id", find_todo_handler);
    router.put("/todos/:id", update_todo_handler);
    router.delete("/todos/:id", delete_todo_handler);
    router.get("/", swagger_handler);
    router.post("/llm", llm_handler);
    router.handle(req)
}
