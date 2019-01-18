use actix_web::http::NormalizePath;
use actix_web::server::IntoHttpHandler;

mod users;
mod tasks;

use crate::SharedState;
use crate::api::ApiApp;
use crate::api::middlewares::ResponseLoggerMiddleware;

pub fn build(state: SharedState) -> impl IntoHttpHandler {
    ApiApp::with_state(state)
        .middleware(ResponseLoggerMiddleware)
        .default_resource(|r| r.h(NormalizePath::default()))
        .configure(users::config)
        .configure(tasks::config)
        .finish()
}
