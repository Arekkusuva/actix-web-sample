use crate::api::prelude::*;
//use crate::api::transport::users::*;

fn get_users(l: Logger, db: Database) -> ResponseResult {
    info!(l, "test logger");
    let u_count = db.users().count()?;
    Ok(Response::new(StatusCode::OK)
        .data(format!("Users count {}", u_count)))
}

pub fn config(mut app: ApiApp) -> ApiApp {
    setup_routes!(app, "users", [
        ("/", Method::POST, get_users),
    ])
}
