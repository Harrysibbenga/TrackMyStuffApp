use diesel::{prelude::*, ConnectionError, PgConnection};
use thiserror::Error; // For the custom error type

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to establish a database connection: {0}")]
    ConnectionError(#[from] ConnectionError),
    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),
}

pub fn establish_connection(database_url: &str) -> Result<PgConnection, DatabaseError> {
    match PgConnection::establish(&database_url) {
        Ok(connection) => Ok(connection),
        Err(e) => Err(DatabaseError::ConnectionError(e)),
    }
}
