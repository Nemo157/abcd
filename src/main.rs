#![feature(async_await)]

use slog::{error, o, Drain};

#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let drain = slog_async::Async::new(slog_bunyan::default(std::io::stdout()).fuse())
        .build()
        .fuse();

    let log = slog::Logger::root(drain, o!("service" => "abcd"));

    let _guard = slog_scope::set_global_logger(log.clone());
    slog_stdlog::init()?;

    std::panic::set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            let payload = info.payload();
            if let Some(message) = payload.downcast_ref::<&str>() {
                error!(
                    slog_scope::logger(),
                    "thread '{thread}' panicked at '{message}', {file}:{line}:{column}",
                    thread = std::thread::current().name().unwrap(),
                    message = message,
                    file = location.file(),
                    line = location.line(),
                    column = location.column(),
                )
            } else {
                error!(
                    slog_scope::logger(),
                    "thread '{thread}' panicked, {file}:{line}:{column}",
                    thread = std::thread::current().name().unwrap(),
                    file = location.file(),
                    line = location.line(),
                    column = location.column(),
                )
            }
        } else {
            unimplemented!()
        }
    }));

    let mut app = tide::App::with_state(abcd::State::new());

    let request_id = || bs58::encode(uuid::Uuid::new_v4().as_bytes()).into_string();

    app.middleware(tide_slog::PerRequestLogger::new(log, move |logger, _cx| {
        logger.new(o!("req_id" => request_id()))
    }));
    app.middleware(tide_slog::SetSlogScopeLogger);
    app.middleware(tide_slog::RequestLogger::new());
    abcd::setup(&mut app);

    let mut listener = runtime::net::TcpListener::bind("127.0.0.1:8000")?;
    http_service_hyper::Server::builder(listener.incoming())
        .with_spawner(runtime::task::Spawner::new())
        .serve(app.into_http_service())
        .await?;

    Ok(())
}
