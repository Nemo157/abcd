#![feature(async_await)]

use slog::{o, Drain};

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let drain = slog_async::Async::new(slog_bunyan::default(std::io::stdout()).fuse()).build().fuse();

    let log = slog::Logger::root(drain, o!("service" => "abcd"));

    let _guard = slog_scope::set_global_logger(log.clone());
    slog_stdlog::init()?;

    let mut app = tide::App::new();

    let request_id = || bs58::encode(uuid::Uuid::new_v4().as_bytes()).into_string();

    app.middleware(tide_slog::PerRequestLogger::new(log, move |logger| {
        logger.new(o!("req_id" => request_id()))
    }));
    app.middleware(tide_slog::SetSlogScopeLogger);
    app.middleware(tide_slog::RequestLogger);
    abcd::setup(&mut app);

    app.serve("127.0.0.1:8000").await?;

    Ok(())
}
