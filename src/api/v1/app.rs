use serde::{Deserialize, Serialize};
use slog::debug;
use tide::{
    forms::ContextExt as _,
    response::{self, IntoResponse},
};
use tide_slog::ContextExt as _;

#[derive(Deserialize, Debug)]
struct RegisterData {
    client_name: String,
    redirect_uris: String,
    scopes: String,
    website: Option<String>,
}

#[derive(Serialize, Debug)]
struct Application {
    name: String,
    website: Option<String>,
    client_id: String,
    client_secret: String,
}

pub async fn register(mut cx: tide::Context<()>) -> tide::EndpointResult<impl IntoResponse> {
    let data: RegisterData = cx.body_form().await?;
    let response = Application {
        name: data.client_name.clone(),
        website: data.website.clone(),
        client_id: "foobar".to_owned(),
        client_secret: "barfoo".to_owned(),
    };
    debug!(cx.logger(), "app register";
           "path" => %cx.uri(),
           "data" => ?data,
           "headers" => ?cx.headers(),
           "response" => ?response);
    Ok(response::json(response))
}
