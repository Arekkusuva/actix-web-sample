use slog::{Logger, Drain};
use std::io::stdout;

pub fn new() -> Logger {
    let plain = slog_term::PlainDecorator::new(stdout());
    let drain = slog_term::FullFormat::new(plain).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = Logger::root(
        drain,
        o!("app" => "web_sample")
    );
    info!(logger, "Logger created => stdout");
    logger
}
