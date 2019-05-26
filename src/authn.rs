use crate::util::{redirect, IntoResponseExt as _};
use serde::{Deserialize, Serialize};
use slog::debug;
use tide::{
    error::ResultExt as _,
    forms::ContextExt as _,
    response::{self, IntoResponse},
};
use tide_slog::ContextExt as _;

#[derive(Deserialize, Debug)]
struct LoginData {
    username: String,
}

#[derive(Deserialize, Debug)]
struct AppRegistrationData {
    client_name: String,
    redirect_uris: String,
    scopes: String,
    website: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AuthorizeData {
    response_type: String,
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

#[derive(Serialize, Debug)]
struct Application {
    name: String,
    website: Option<String>,
    client_id: String,
    client_secret: String,
}

pub async fn login(mut cx: tide::Context<()>) -> tide::EndpointResult<impl IntoResponse> {
    let data: LoginData = cx.body_form().await?;
    debug!(cx.logger(), "login";
           "path" => %cx.uri(),
           "data" => ?data,
           "headers" => ?cx.headers());
    Ok(format!("Hello, {}!", data.username))
}

pub async fn register_app(mut cx: tide::Context<()>) -> tide::EndpointResult<impl IntoResponse> {
    let data: AppRegistrationData = cx.body_form().await?;
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

pub async fn authorize(cx: tide::Context<()>) -> tide::EndpointResult<impl IntoResponse> {
    let data: AuthorizeData = serde_urlencoded::from_str(cx.uri().query().unwrap()).client_err()?;
    let location = data.redirect_uri.as_ref().unwrap().clone() + "?code=bazfoo";
    debug!(cx.logger(), "oauth authorize";
           "path" => %cx.uri(),
           "data" => ?data,
           "redirect" => ?location,
           "headers" => ?cx.headers());
    Ok("Redirecting...".with(redirect(&location).server_err()?))
}
