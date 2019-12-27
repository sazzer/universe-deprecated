use super::ServiceCreationError;
use crate::database::{
    postgres::{PostgresDatabase, PostgresDatabaseError},
    Database,
};
use std::sync::Arc;

impl From<PostgresDatabaseError> for ServiceCreationError {
    fn from(e: PostgresDatabaseError) -> Self {
        Self {
            message: format!("Error connecting to database: {}", e),
        }
    }
}

/// Connects to the database for other services to use
///
/// # Arguments
/// # `database_url` The connection URL for the database
///
/// # Returns
/// The database connection
pub fn new(database_url: String) -> Result<Arc<dyn Database>, ServiceCreationError> {
    let database: Arc<dyn Database> = Arc::new(PostgresDatabase::new(database_url)?);

    Ok(database)
}
