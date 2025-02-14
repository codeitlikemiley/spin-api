use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub(crate) struct Todo {
    pub(super) id: u64,
    pub(super) title: String,
    pub(super) completed: bool,
}

#[derive(ToSchema, Serialize, Deserialize)]
pub(crate) struct LlmResponse {
    pub(super) response: String,
}
