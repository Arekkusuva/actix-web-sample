#[macro_use]
extern crate slog;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
mod macros;
mod logger;
mod api;
mod db;

use std::sync::Arc;

use self::db::Database;

#[derive(Clone)]
pub struct AppState {
    pub logger: slog::Logger,
    pub db: Database,
}

pub type SharedState = Arc<AppState>;

fn main() {
    let logger = logger::new();
    let db = Database::new("postgresql://localhost/actixwebsampledb?user=testuser&password=password");
    let app_state = AppState { logger, db };

    api::run(SharedState::new(app_state))
}
