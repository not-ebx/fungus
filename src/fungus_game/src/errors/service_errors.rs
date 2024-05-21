use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServiceError {
    NotFound,
    Unauthorized,
    NoPermission,
    InvalidDetails,

    InvalidDatabaseConnection,
    GenericDatabaseError(String)
}


impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::NotFound => write!(f, "Could not fetch"),
            ServiceError::Unauthorized => write!(f, "Not authorized to perform this action"),
            ServiceError::NoPermission => write!(f, "You don't have the needed permissions to perform this action"),
            ServiceError::InvalidDetails => write!(f, "Incorrect details were specified for this action"),
            ServiceError::InvalidDatabaseConnection => write!(f, "Could NOT connect to database."),
            ServiceError::GenericDatabaseError(err) => write!(f, "{}", err)
        }
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ServiceError::NotFound,
            sqlx::Error::PoolTimedOut | sqlx::Error::Io(..) | sqlx::Error::PoolClosed => {
                ServiceError::InvalidDatabaseConnection
            }
            _ => ServiceError::GenericDatabaseError(value.to_string())
            /*
            Error::TypeNotFound { .. } => {}
            Error::ColumnIndexOutOfBounds { .. } => {}
            Error::ColumnNotFound(_) => {}
            Error::ColumnDecode { .. } => {}
            Error::Decode(_) => {}
            Error::AnyDriverError(_) => {}
            Error::PoolTimedOut => {}
            Error::PoolClosed => {}
            Error::WorkerCrashed => {}
            Error::Migrate(_) => {}
             */
        }
    }
}