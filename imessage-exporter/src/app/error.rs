/*!
Errors that can happen during the application's runtime
*/

use std::{
    fmt::{Display, Formatter, Result},
    io::Error as IoError,
};

use imessage_database::error::table::TableError;

/// Errors that can happen during the application's runtime
#[derive(Debug)]
pub enum RuntimeError {
    InvalidOptions(String),
    DiskError(IoError),
    DatabaseError(TableError),
}

impl Display for RuntimeError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            RuntimeError::InvalidOptions(why) => write!(fmt, "Invalid options!\n{why}"),
            RuntimeError::DiskError(why) => write!(fmt, "{why}"),
            RuntimeError::DatabaseError(why) => write!(fmt, "{why}"),
        }
    }
}
