//! [infrastructure][crate] defines concrete adapters/implementations for the
//! ports exposed by [application].
//!
//! [application]: ../application

pub mod user;
pub mod vendor;

pub(crate) enum PgError {
    Unknown(Box<dyn std::error::Error>),
}

impl From<sqlx::Error> for PgError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            _ => PgError::Unknown(Box::new(err)),
        }
    }
}
