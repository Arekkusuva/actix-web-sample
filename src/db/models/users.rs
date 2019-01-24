use diesel::prelude::*;
use diesel::insert_into;

use bcrypt::{DEFAULT_COST, hash};

use crate::db::{Database, DbResult, DbError};
use crate::db::schema::users;

pub struct Users<'a> {
    db: &'a Database,
}

impl<'a> Users<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn count(&self) -> DbResult<i64> {
        users::table.count().get_result(&self.db.conn()).map_err(DbError::from)
    }

    pub fn create(&self, email: &str, pwd: &str) {
        let hashed_pwd = hash(pwd, DEFAULT_COST).unwrap();
        insert_into(users::table)
            .values((
                users::email.eq(email),
                users::password.eq(hashed_pwd),
            ))
            .returning(users::id)
            .get_result::<i32>(&self.db.conn()).unwrap();
    }
}
