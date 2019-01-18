use actix_web::Responder;
use actix_web::http::Method;

use crate::api::{ApiApp, Request};

fn get_tasks(_req: &Request) -> impl Responder {
    format!("GET tasks")
}

pub fn config(app: ApiApp) -> ApiApp {
    app
        .resource("/", |r| r.method(Method::GET).f(get_tasks))
}