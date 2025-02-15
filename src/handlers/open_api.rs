use spin_sdk::http::{Params, Request, Response};
use utoipa::OpenApi;

use crate::api::Api;

pub(crate) async fn openapi_handler(_req: Request, _params: Params) -> anyhow::Result<Response> {
    let doc = Api::openapi().to_pretty_json().unwrap();
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(doc)
        .build())
}
