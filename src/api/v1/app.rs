use oxide_auth::primitives::registrar::Client;
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

pub async fn register(
    mut cx: tide::Context<crate::State>,
) -> tide::EndpointResult<impl IntoResponse> {
    let data: RegisterData = cx.body_form().await?;
    let response = Application {
        name: data.client_name.clone(),
        website: data.website.clone(),
        client_id: "foobar".to_owned(),
        client_secret: "barfoo".to_owned(),
    };
    let client = Client::confidential(
        &response.client_id,
        data.redirect_uris
            .split(",")
            .next()
            .unwrap()
            .parse()
            .unwrap(),
        data.scopes.parse().unwrap(),
        b"barfoo",
    );
    debug!(cx.logger(), "app register";
           "path" => %cx.uri(),
           "data" => ?data,
           "client" => ?client,
           "headers" => ?cx.headers(),
           "response" => ?response);
    cx.state().registrar.lock().unwrap().register_client(client);
    Ok(response::json(response))
}
