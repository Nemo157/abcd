#![feature(async_await, type_alias_enum_variants)]

mod oauth;
mod util;
mod api;

pub fn setup(app: &mut tide::App<()>) {
    app.at("/").get(async move |_| "Hello, world!");

    app.at("/api/v1/apps").post(api::v1::app::register);
    app.at("/api/v1/accounts/verify_credentials").get(api::v1::account::verify_credentials);

    app.at("/oauth/authorize").get(oauth::authorize);
    app.at("/oauth/token").post(oauth::token);
}
