#[macro_use]
extern crate slog;
#[macro_use]
extern crate diesel;
#[macro_use]
mod macros;
mod logger;
mod api;
mod db;

use slog::Logger;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub logger: Logger,
}

pub type SharedState = Arc<AppState>;

fn main() {
    let app_state = AppState {
        logger: logger::new(),
    };
    api::run(SharedState::new(app_state))
}
