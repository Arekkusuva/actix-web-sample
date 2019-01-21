use diesel::pg::PgConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager, Pool};

pub mod models;
pub mod schema;

use std::fmt;

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: DBPool,
}

impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database {{ DBPool: {:?} }}", self.pool.state())
    }
}

impl Database {
    pub fn new<S: Into<String>>(uri: S) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(uri);
        let pool = Pool::new(manager).expect("Create diesel pool");

        Self { pool }
    }

    pub fn conn(&self) -> DBConnection {
        self.pool.get().unwrap()
    }

    pub fn users(&self) -> models::Users {
        models::Users::new(self)
    }
}
