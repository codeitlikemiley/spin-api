use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct CreateTodo {
    pub(crate) title: String,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct UpdateTodo {
    pub(crate) title: Option<String>,
    pub(crate) completed: Option<bool>,
}
