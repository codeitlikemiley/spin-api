use utoipa::OpenApi;

use crate::{CreateTodo, Todo, UpdateTodo};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Todo API",
        version = "0.1.0",
        contact(name = "Uriah", email = "codeitlikemiley@gmail.com"),
        license(name = "MIT")
    ),
    paths(
        crate::handlers::todos::list_todos_handler,
        crate::handlers::todos::create_todo_handler,
        crate::handlers::todos::find_todo_handler,
        crate::handlers::todos::update_todo_handler,
        crate::handlers::todos::delete_todo_handler
    ),
    components(schemas(Todo, CreateTodo, UpdateTodo))
)]
pub(crate) struct Api;
