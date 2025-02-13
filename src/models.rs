use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub(crate) struct Todo {
    pub(crate) id: u64,
    pub(crate) title: String,
    pub(crate) completed: bool,
}
