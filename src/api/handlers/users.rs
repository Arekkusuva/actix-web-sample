use crate::api::prelude::*;
use crate::api::transport::users::*;
use crate::db::models::users::NewUser;

fn post_user(body: Json<PostUser>, l: Logger, db: Database) -> ResponseResult {
    let db_users = db.users();
    if !db_users.email_exists(&body.email)? {
        let new_user = NewUser {
            email: &body.email,
            password: &body.password,
        };

        let user = db.users().create(&new_user)?;

        return Ok(Response::new(StatusCode::CREATED)
            .data(format!("id = {}", user.id)))
    }

    Ok(Response::new(StatusCode::CONFLICT))
}

pub fn config(mut app: ApiApp) -> ApiApp {
    setup_routes!(app, "users", [
        ("/", Method::POST, post_user),
    ])
}
