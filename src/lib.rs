#![feature(async_await, type_alias_enum_variants)]

pub fn setup(app: &mut tide::App<()>) {
    app.at("/").get(async move |_| "Hello, world!");
}
