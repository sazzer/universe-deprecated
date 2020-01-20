use crate::{migrate::migrate, postgres::PostgresDatabase, Database};
use std::sync::Arc;

/// Connects to the database for other services to use
///
/// # Arguments
/// # `database_url` The connection URL for the database
///
/// # Returns
/// The database connection
pub fn new(database_url: &str, migration_files: &str) -> Result<Arc<dyn Database>, String> {
    let postgres_database = PostgresDatabase::new(database_url)
        .map_err(|e| format!("Error connecting to database: {}", e))?;
    let database: Arc<dyn Database> = Arc::new(postgres_database);

    migrate(database.clone(), migration_files)
        .map_err(|e| format!("Error migrating database: {}", e))?;
    Ok(database)
}
