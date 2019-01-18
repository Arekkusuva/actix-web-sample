use slog::Logger;

use crate::api::Request;

pub trait LoggerReqExt {
    fn root_logger(&self) -> Logger;
    fn logger(&self) -> Logger;
}

impl LoggerReqExt for Request {
    fn root_logger(&self) -> Logger {
        self.state().logger.clone()
    }

    fn logger(&self) -> Logger {
        self.state().logger.new(o!(
            "module" => "LoggerReqExt",
            "route" => format!("{} {}", self.method().as_str(), self.path()),
        ))
    }
}
