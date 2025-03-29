use std::io::Error as IoError;

use actix_error_proc::ActixError;
use flexi_logger::FlexiLoggerError;
use thiserror::Error as ThisError;

pub mod extractors;
pub mod routes;

#[derive(Debug, ThisError, ActixError)]
pub enum AppError {
    #[error("{0:#}")]
    Io(#[from] IoError),

    #[error("0:#")]
    LoggerError(#[from] FlexiLoggerError),

    #[http_status(Unauthorized)]
    #[error("You are not authorized to access this resource")]
    AuthorizationError,
}
