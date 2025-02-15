use serde::Deserialize;
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize)]
pub(crate) struct CreateTodo {
    pub(crate) title: String,
}

#[derive(ToSchema, Deserialize)]
pub(crate) struct UpdateTodo {
    pub(crate) title: Option<String>,
    pub(crate) completed: Option<bool>,
}
