use crate::api::prelude::*;
use crate::SharedState;

fn get_users(req: &Request) -> impl Responder {
    let l = req.logger();
    let count = (req.state() as &SharedState).db.users().count();
    format!("Users count {:?}", count)
}

pub fn config(mut app: ApiApp) -> ApiApp {
    setup_routes!(app, "users", [
        ("/", Method::GET, get_users),
        ("/g", Method::GET, get_users),
    ])
}
