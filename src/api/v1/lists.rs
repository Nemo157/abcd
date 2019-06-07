use serde::Serialize;
use slog::debug;
use tide::response::{self, IntoResponse};
use tide_slog::ContextExt as _;

#[derive(Serialize, Debug)]
struct List {
    id: String,
    title: String,
}

pub async fn mine(
    cx: tide::Context<crate::State>,
) -> tide::EndpointResult<impl IntoResponse> {
    let response = vec![List {
        id: "5".into(),
        title: "my list".into(),
    }];
    debug!(cx.logger(), "mine";
           "path" => %cx.uri(),
           "response" => ?response,
           "headers" => ?cx.headers());
    Ok(response::json(response))
}
