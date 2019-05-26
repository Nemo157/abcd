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
struct AuthorizeData {
    response_type: String,
    client_id: String,
    redirect_uri: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

#[derive(Deserialize, Debug)]
struct TokenData {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,

    client_secret: String,
}

#[derive(Serialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
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

pub async fn token(mut cx: tide::Context<()>) -> tide::EndpointResult<impl IntoResponse> {
    let data: serde_json::Value = cx.body_form().await?;
    debug!(cx.logger(), "oauth token data"; "data" => ?data);
    let data: TokenData = serde_json::from_value(data).client_err()?;
    let response = TokenResponse {
        access_token: "foobaz".to_owned(),
        token_type: "bearer".to_owned(),
    };
    debug!(cx.logger(), "oauth token";
           "path" => %cx.uri(),
           "data" => ?data,
           "response" => ?response,
           "headers" => ?cx.headers());
    Ok(response::json(response))
}
