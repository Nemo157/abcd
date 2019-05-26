#![feature(async_await, type_alias_enum_variants)]

mod authn;
mod util;

use util::IntoResponseExt as _;

pub fn setup(app: &mut tide::App<()>) {
    app.at("/").get(async move |_| "Hello, world!");

    app.at("/login")
        .get(async move |_| include_str!("login.html").with(mime::TEXT_HTML))
        .post(authn::login);

    app.at("/api/v1/apps").post(authn::register_app);
    app.at("/api/v1/accounts/verify_credentials").get(authn::verify);

    app.at("/oauth/authorize").get(authn::authorize);
    app.at("/oauth/token").post(authn::token);
}
