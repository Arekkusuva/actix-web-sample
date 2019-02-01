use diesel::prelude::*;
use diesel;
use diesel::dsl::exists;

use crate::db::{Database, DbResult};
use crate::db::schema::users;

pub struct NewUser<'a> {
    pub email: &'a str,
    pub hashed_password: &'a str,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hashed_password: String,
}

pub struct Users<'a> {
    db: &'a Database,
}

impl<'a> Users<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn count(&self) -> DbResult<i64> {
        Ok(users::table
            .count()
            .get_result(&self.db.conn())?)
    }

    pub fn email_exists(&self, email: &str) -> DbResult<bool> {
        Ok(diesel::select(exists(
            users::table.filter(users::email.eq(email))))
            .get_result(&self.db.conn())?)
    }

    pub fn create(&self, user: &NewUser) -> DbResult<User> {
        Ok(diesel::insert_into(users::table)
            .values((
                users::email.eq(user.email),
                users::hashed_password.eq(user.hashed_password),
            ))
            .get_result(&self.db.conn())?)
    }
}
