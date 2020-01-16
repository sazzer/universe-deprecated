use super::ServiceCreationError;
use std::sync::Arc;
use universe_database::{
    migrate::{migrate, MigrationError},
    postgres::{PostgresDatabase, PostgresDatabaseError},
    Database,
};

impl From<PostgresDatabaseError> for ServiceCreationError {
    fn from(e: PostgresDatabaseError) -> Self {
        Self {
            message: format!("Error connecting to database: {}", e),
        }
    }
}

impl From<MigrationError> for ServiceCreationError {
    fn from(e: MigrationError) -> Self {
        Self {
            message: format!("Error migrating database: {}", e),
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
pub fn new(
    database_url: String,
    migration_files: String,
) -> Result<Arc<dyn Database>, ServiceCreationError> {
    let database: Arc<dyn Database> = Arc::new(PostgresDatabase::new(database_url)?);

    migrate(database.clone(), migration_files)?;
    Ok(database)
}