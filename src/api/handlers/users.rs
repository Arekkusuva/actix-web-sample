use actix_web::Responder;
use actix_web::http::Method;

use crate::api::prelude::*;
use crate::api::{ApiApp, Request};

fn get_users(req: &Request) -> impl Responder {
    let l = req.logger();
    info!(l, "test log");
    format!("GET users")
}

pub fn config(mut app: ApiApp) -> ApiApp {
    setup_routes!(app, "users", [
        ("/", Method::GET, get_users),
        ("/g", Method::GET, get_users),
    ])
}
