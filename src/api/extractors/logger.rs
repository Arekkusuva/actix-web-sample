use actix_web::{HttpRequest, FromRequest};
use slog::Logger as SlogLogger;

use std::ops::Deref;

use crate::SharedState;

pub struct Logger(SlogLogger);

impl Deref for Logger {
    type Target = SlogLogger;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest<SharedState> for Logger {
    type Config = ();
    type Result = Self;

    #[inline]
    fn from_request(req: &HttpRequest<SharedState>, _: &Self::Config) -> Self::Result {
        Logger(req.state().logger.new(o!(
            "module" => "LoggerReqExt",
            "route" => format!("{} {}", req.method().as_str(), req.path()),
        )))
    }
}
