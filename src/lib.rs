#![feature(async_await, type_alias_enum_variants)]

use http::Response;
use serde::{Deserialize, Serialize};
use serde_json;
use slog::debug;
use tide::{error::ResultExt as _, forms::ContextExt as _, response::IntoResponse};
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

struct Html(&'static str);

impl IntoResponse for Html {
    fn into_response(self) -> Response<http_service::Body> {
        Response::builder()
            .header("Content-Type", "text/html")
            .body(self.0.into())
            .unwrap()
    }
}

struct Json<T: Serialize>(T);

impl<T: Serialize + Send> IntoResponse for Json<T> {
    fn into_response(self) -> Response<http_service::Body> {
        Response::builder()
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&self.0).unwrap().into())
            .unwrap()
    }
}

struct Redirect(String);

impl IntoResponse for Redirect {
    fn into_response(self) -> Response<http_service::Body> {
        Response::builder()
            .header("Location", self.0)
            .status(302)
            .body("Redirecting...".into())
            .unwrap()
    }
}

pub fn setup(app: &mut tide::App<()>) {
    app.at("/").get(async move |_| "Hello, world!");

    app.at("/login")
        .get(async move |_| Html(include_str!("login.html")))
        .post(async move |mut cx: tide::Context<()>| {
            let data: LoginData = cx.body_form().await?;
            debug!(cx.logger(), "login";
                   "path" => %cx.uri(),
                   "data" => ?data,
                   "headers" => ?cx.headers());
            tide::EndpointResult::Ok(format!("Hello, {}!", data.username))
        });

    app.at("/api/v1/apps")
        .post(async move |mut cx: tide::Context<()>| {
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
            tide::EndpointResult::Ok(Json(response))
        });

    app.at("/oauth/authorize")
        .get(async move |cx: tide::Context<()>| {
            let data: AuthorizeData =
                serde_urlencoded::from_str(cx.uri().query().unwrap()).client_err()?;
            let redirect = data.redirect_uri.as_ref().unwrap().clone() + "?code=bazfoo";
            debug!(cx.logger(), "oauth authorize";
                   "path" => %cx.uri(),
                   "data" => ?data,
                   "redirect" => ?redirect,
                   "headers" => ?cx.headers());
            tide::EndpointResult::Ok(Redirect(redirect))
        });
}
