#![feature(async_await, type_alias_enum_variants)]

use http::Response;
use serde::Deserialize;
use slog::debug;
use tide::{forms::ContextExt as _, response::IntoResponse};
use tide_slog::ContextExt as _;

#[derive(Deserialize, Debug)]
struct LoginData {
    username: String,
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

pub fn setup(app: &mut tide::App<()>) {
    app.at("/").get(async move |_| "Hello, world!");
    app.at("/login")
        .get(async move |_| Html(include_str!("login.html")))
        .post(async move |mut cx: tide::Context<()>| {
            let data = cx.body_form::<LoginData>().await?;
            debug!(cx.logger(), "login";
                   "path" => %cx.uri(),
                   "data" => ?data,
                   "headers" => ?cx.headers());
            tide::EndpointResult::<String>::Ok(format!("Hello, {}!", data.username))
        });
}
