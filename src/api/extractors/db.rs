use actix_web::{HttpRequest, FromRequest};
use crate::db::Database as Db;

use std::ops::Deref;

use crate::SharedState;

pub struct Database(Db);

impl Deref for Database {
    type Target = Db;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest<SharedState> for Database {
    type Config = ();
    type Result = Self;

    #[inline]
    fn from_request(req: &HttpRequest<SharedState>, _: &Self::Config) -> Self::Result {
        Database(req.state().db.clone())
    }
}
