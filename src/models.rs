use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Clone)]
pub(crate) struct Todo {
    pub(super) id: u64,
    pub(super) title: String,
    pub(super) completed: bool,
}

impl Todo {
    pub(super) fn from_row(row: &spin_sdk::sqlite::Row) -> anyhow::Result<Self> {
        Ok(Self {
            id: row
                .get::<u64>("id")
                .ok_or_else(|| anyhow::anyhow!("Missing column: id"))?,
            title: row
                .get::<&str>("title")
                .ok_or_else(|| anyhow::anyhow!("Missing column: title"))?
                .to_string(),
            completed: row
                .get::<bool>("completed")
                .ok_or_else(|| anyhow::anyhow!("Missing column: completed"))?,
        })
    }
}
