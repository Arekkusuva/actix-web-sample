use crate::api::prelude::*;

fn get_tasks(_req: &Request) -> impl Responder {
    format!("GET tasks")
}

pub fn config(mut app: ApiApp) -> ApiApp {
    setup_routes!(app, "tasks", [
        ("/", Method::GET, get_tasks),
    ])
}