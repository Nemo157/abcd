#![feature(async_await)]

use slog::{o, Drain};

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> std::io::Result<()> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("service" => "abcd"));

    let mut app = tide::App::new();

    app.middleware(tide_slog::RequestLogger::with_logger(log));
    abcd::setup(&mut app);

    app.serve("127.0.0.1:8000").await?;

    Ok(())
}
