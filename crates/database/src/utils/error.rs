use actix_error_proc::ActixError;
use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;

use super::connection::DatabaseConnectionError;

pub type ModelResult<T> = Result<T, DatabaseError>;

#[derive(ThisError, Debug, ActixError)]
pub enum DatabaseError {
    #[error("{0:#}")]
    DatabaseQuery(#[from] SqlxError),

    #[error("{0:#}")]
    DatabaseConnectionError(#[from] DatabaseConnectionError),

    #[http_status(NotFound)]
    #[error("No {0} found with that query.")]
    ModelNotFound(&'static str),
}
