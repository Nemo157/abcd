#![feature(async_await, type_alias_enum_variants)]

#[macro_use]
extern crate diesel;

use std::sync::Mutex;

mod api;
mod oauth;
mod util;

mod schema;

pub struct State {
    registrar: Mutex<oxide_auth::primitives::registrar::ClientMap>,
    authorizer: Mutex<
        oxide_auth::primitives::authorizer::AuthMap<
            oxide_auth::primitives::generator::RandomGenerator,
        >,
    >,
    issuer: Mutex<oxide_auth::primitives::issuer::TokenSigner>,
}

impl State {
    pub fn new() -> Self {
        Self {
            registrar: Mutex::new(oxide_auth::primitives::registrar::ClientMap::new()),
            authorizer: Mutex::new(oxide_auth::primitives::authorizer::AuthMap::new(
                oxide_auth::primitives::generator::RandomGenerator::new(16),
            )),
            issuer: Mutex::new(oxide_auth::primitives::issuer::TokenSigner::ephemeral()),
        }
    }
}

pub fn setup(app: &mut tide::App<State>) {
    app.at("/").get(async move |_| "Hello, world!");

    app.at("/api/v1/apps").post(api::v1::app::register);
    app.at("/api/v1/lists").get(api::v1::lists::mine);
    app.at("/api/v1/accounts/verify_credentials")
        .get(api::v1::account::verify_credentials);

    app.at("/oauth/authorize").get(oauth::authorize);
    app.at("/oauth/token").post(oauth::token);
}
