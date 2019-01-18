use diesel::pg::PgConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager, Pool};
use slog::Logger;

pub mod models;

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new<S: Into<String>>(uri: S) -> Self {

        let manager = ConnectionManager::<PgConnection>::new(uri);
        let pool = Pool::new(manager).expect("Create diesel pool");

        Self { pool }
    }
}
