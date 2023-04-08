use std::io;

use sqlx::Error as SqlError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Cannot connect to database")]
    DBError(#[from] SqlError),
    #[error("IO Error")]
    IOError(#[from] io::Error),
}

pub type ServerResult<T> = Result<T, ServerError>;
