use actix_web::App;
use actix_web::server;

pub mod prelude;
pub mod transport;

mod handlers;
mod middlewares;
mod extractors;
mod errors;

use crate::SharedState;

pub type ApiApp = App<SharedState>;

pub fn run(state: SharedState) {
    server::new(move || handlers::build(state.clone()))
        .bind("127.0.0.1:8000")
        .expect("binding")
        .run();
}
