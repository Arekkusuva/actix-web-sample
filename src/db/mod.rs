use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind;
use diesel::pg::PgConnection;
use diesel::r2d2::{PooledConnection, ConnectionManager, Pool};
use bcrypt::BcryptError;

pub mod models;
pub mod schema;

use std::fmt;
use std::convert::From;
use std::error;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub enum DbError {
    AlreadyExist,
    NotFound,
    ForeignKeyViolation,
    CryptoError(BcryptError),
    UnrecognizedDatabaseError(DieselError),
}

pub type DbResult<T> = std::result::Result<T, DbError>;

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::NotFound => f.write_str("NotFound"),
            DbError::AlreadyExist => f.write_str("AlreadyExist"),
            DbError::ForeignKeyViolation => f.write_str("ForeignKeyViolation"),
            DbError::CryptoError(ref e) => e.fmt(f),
            DbError::UnrecognizedDatabaseError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DbError {}

impl From<DieselError> for DbError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => DbError::NotFound,
            DieselError::DatabaseError(kind, _) => match kind {
                DatabaseErrorKind::ForeignKeyViolation => DbError::ForeignKeyViolation,
                DatabaseErrorKind::UnableToSendCommand => DbError::AlreadyExist,
                _ => DbError::UnrecognizedDatabaseError(err),
            },
            _ => DbError::UnrecognizedDatabaseError(err),
        }
    }
}

impl From<BcryptError> for DbError {
    fn from(err: BcryptError) -> Self {
        DbError::CryptoError(err)
    }
}

#[derive(Clone)]
pub struct Database {
    pool: DbPool,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Database {{ DbPool: {:?} }}", self.pool.state())
    }
}

impl Database {
    pub fn new<S: Into<String>>(uri: S) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(uri);
        let pool = Pool::new(manager).expect("Create diesel pool");

        Self { pool }
    }

    #[inline]
    pub fn conn(&self) -> DbConnection {
        self.pool.get().unwrap()
    }

    pub fn users(&self) -> models::Users {
        models::Users::new(self)
    }
}
