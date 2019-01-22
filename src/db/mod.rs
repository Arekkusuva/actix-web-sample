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

//use crate::api::Request;

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub enum Error {
    AlreadyExist,
    NotFound,
    ForeignKeyViolation,
    CryptoError(BcryptError),
    UnrecognizedDatabaseError(DieselError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound => f.write_str("NotFound"),
            Error::AlreadyExist => f.write_str("AlreadyExist"),
            Error::ForeignKeyViolation => f.write_str("ForeignKeyViolation"),
            Error::CryptoError(ref e) => e.fmt(f),
            Error::UnrecognizedDatabaseError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => Error::NotFound,
            DieselError::DatabaseError(kind, _) => match kind {
                DatabaseErrorKind::ForeignKeyViolation => Error::ForeignKeyViolation,
                DatabaseErrorKind::UnableToSendCommand => Error::AlreadyExist,
                _ => Error::UnrecognizedDatabaseError(err),
            },
            _ => Error::UnrecognizedDatabaseError(err),
        }
    }
}

impl From<BcryptError> for Error {
    fn from(err: BcryptError) -> Self {
        Error::CryptoError(err)
    }
}

#[derive(Clone)]
pub struct Database {
    pool: DBPool,
}

impl std::fmt::Debug for Database {
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

    #[inline]
    pub fn conn(&self) -> DBConnection {
        self.pool.get().unwrap()
    }

    pub fn users(&self) -> models::Users {
        models::Users::new(self)
    }
}
