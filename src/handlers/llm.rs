use spin_sdk::{
    http::{IntoResponse, Params, Request, Response},
    llm::{self, InferencingModel, InferencingResult},
};

use crate::{models::LlmResponse, request::Message};

#[utoipa::path(
    post,
    path = "/llm",
    tags = ["LLM"],
    request_body = Message,
    responses(
        (status = 200, description = "Chat", body = LlmResponse)
    )
)]
pub(crate) fn llm_handler(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let model = InferencingModel::Llama2Chat;
    let message: Message = serde_json::from_slice(req.body())?;
    let inference: InferencingResult = llm::infer(model, message.body).unwrap();

    let response = serde_json::to_string(&LlmResponse {
        response: inference.text,
    })?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(response)
        .build())
}
