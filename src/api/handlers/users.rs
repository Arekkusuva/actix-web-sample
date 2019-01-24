use crate::api::prelude::*;
use crate::api::transport::users::*;

fn get_users(body: Json<PostUser>, l: Logger, db: Database) -> ResponseResult {
    info!(l, "body is: {:?}", body);
    let u_count = db.users().count()?;
    Ok(Response::new(StatusCode::OK)
        .data(format!("Users count {}", u_count)))
}

pub fn config(mut app: ApiApp) -> ApiApp {
    setup_routes!(app, "users", [
        ("/", Method::POST, get_users),
    ])
}
