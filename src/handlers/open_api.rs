use spin_sdk::http::{IntoResponse, Params, Request, Response};
use utoipa::OpenApi;

use crate::api::Api;

pub(crate) fn openapi_handler(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let doc = Api::openapi().to_pretty_json().unwrap();
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(doc)
        .build())
}
