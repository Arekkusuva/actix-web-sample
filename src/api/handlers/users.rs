use crate::api::prelude::*;
use crate::api::transport::users::*;
use crate::db::models::users::NewUser;
use bcrypt::hash;

fn post_user(body: Json<PostUser>, _l: Logger, db: Database) -> ResponseResult {
    let db_users = db.users();
    if db_users.email_exists(&body.email)? {
        return Ok(Response::new(StatusCode::CONFLICT));
    }

    let hashed_pwd = hash(&body.password, 10)?;

    let user = db.users().create(&NewUser {
        email: &body.email,
        hashed_password: &hashed_pwd,
    })?;

    Ok(Response::new(StatusCode::CREATED)
        .data(format!("id = {}", user.id)))
}

pub fn config(mut app: ApiApp) -> ApiApp {
    setup_routes!(app, "users", [
        ("/", Method::POST, post_user),
    ])
}
