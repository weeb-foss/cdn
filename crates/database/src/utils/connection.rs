use std::path::Path;
use std::sync::OnceLock;

use sqlx::migrate::{MigrateError, Migrator};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as SqlxError, Pool, Postgres};
use thiserror::Error as ThisError;

/// This macro obtains a connection to the database,
/// beware it must be in an async (sugar syntaxed) context.
#[macro_export]
macro_rules! db {
    () => {
        $crate::utils::connection::get_db_connection().await?
    };
}

static CONNECTION: OnceLock<Pool<Postgres>> = OnceLock::new();

#[derive(ThisError, Debug)]
pub enum DatabaseConnectionError {
    #[error("{0:#}")]
    ConnectionError(#[from] SqlxError),

    #[error("{0:#}")]
    MigrateError(#[from] MigrateError),
}

/// This obtains a database connection from the `CONNECTION` oncelock
/// or creates a new connection and stores it there.
///
/// This is not to be used directly, prefer `db!()` instead.
pub async fn get_db_connection<'r>() -> Result<&'r Pool<Postgres>, DatabaseConnectionError> {
    if let Some(connection) = CONNECTION.get() {
        return Ok(connection);
    }

    let pool = PgPoolOptions::new() //
        .max_connections(5)
        .connect(env!("DATABASE_URL"))
        .await?;

    Migrator::new(Path::new(env!("DATABASE_MIGRATIONS"))) //
        .await?
        .run(&pool)
        .await?;

    Ok(CONNECTION.get_or_init(|| pool))
}
