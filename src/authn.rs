use crate::util::{redirect, IntoResponseExt as _};
use serde::{Deserialize, Serialize};
use slog::debug;
use tide::{
    error::ResultExt as _,
    forms::ContextExt as _,
    response::{self, IntoResponse},
};
use tide_slog::ContextExt as _;
use chrono::{DateTime, Utc};

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

#[derive(Serialize, Debug)]
struct Application {
    name: String,
    website: Option<String>,
    client_id: String,
    client_secret: String,
}

#[derive(Serialize, Debug)]
struct Account {
    id: String,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    created_at: DateTime<Utc>,
    followers_count: u64,
    following_count: u64,
    statuses_count: u64,
    note: String,
    url: String,
    avatar: String,
    avatar_static: String,
    header: String,
    header_static: String,
    emojis: Vec<()>,
    moved: Option<()>,
    fields: Option<()>,
    bot: Option<()>,
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

pub async fn verify(cx: tide::Context<()>) -> tide::EndpointResult<impl IntoResponse> {
    let response = Account {
        id: "5".into(),
        username: "username".into(),
        acct: "acct".into(),
        display_name: "display_name".into(),
        locked: false,
        created_at: Utc::now(),
        followers_count: 0,
        following_count: 0,
        statuses_count: 0,
        note: "".into(),
        url: "".into(),
        avatar: "".into(),
        avatar_static: "".into(),
        header: "".into(),
        header_static: "".into(),
        emojis: vec![],
        moved: None,
        fields: None,
        bot: None,
    };
    debug!(cx.logger(), "verify credentials";
           "path" => %cx.uri(),
           "response" => ?response,
           "headers" => ?cx.headers());
    Ok(response::json(response))
}
