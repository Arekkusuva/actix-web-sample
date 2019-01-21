use actix_web::http::{NormalizePath, StatusCode};
use actix_web::server::IntoHttpHandler;

mod users;
mod tasks;

use crate::SharedState;
use crate::api::ApiApp;
use crate::api::middlewares::ResponseLoggerMiddleware;

pub fn build(state: SharedState) -> impl IntoHttpHandler {
//    use std::collections::HashMap;
//    let mut r = HashMap::new();
//    r.entry("s")
//        .or_insert_with(|| Vec::with_capacity(1))
//        .push(("a", "b"));
//
//    for (a, handlers) in r {
//        for (method, handler) in handlers {
//
//        }
//    }

    ApiApp::with_state(state)
        .middleware(ResponseLoggerMiddleware)
        .default_resource(|r| r.h(NormalizePath::new(false, true, StatusCode::MOVED_PERMANENTLY)))
        .configure(users::config)
        .configure(tasks::config)
        .finish()
}
