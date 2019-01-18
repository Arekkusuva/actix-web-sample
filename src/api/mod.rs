use actix_web::{App, HttpRequest};
use actix_web::server;

pub mod prelude;

mod handlers;
mod middlewares;

use crate::SharedState;

type ApiApp = App<SharedState>;
type Request = HttpRequest<SharedState>;

pub fn run(state: SharedState) {
    server::new(move || handlers::build(state.clone()))
        .bind("127.0.0.1:8000")
        .expect("binding")
        .run();
}
