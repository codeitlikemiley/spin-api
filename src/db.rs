use anyhow::Context;
use spin_sdk::sqlite::Connection;

pub(crate) async fn db_connection() -> anyhow::Result<Connection> {
    Connection::open_default().context("Failed to open database connection")
}
