use diesel::prelude::*;
use diesel::QueryResult;

use crate::db::Database;
use crate::db::schema::users::dsl::*;

pub struct Users<'a> {
    db: &'a Database,
}

impl<'a> Users<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn count(&self) -> QueryResult<i64> {
        users.count().get_result(&self.db.conn())
    }
}
