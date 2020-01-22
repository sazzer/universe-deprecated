use crate::{migrate::MigratableDatabase, postgres::Database};

/// Connects to the database for other services to use
///
/// # Arguments
/// # `database_url` The connection URL for the database
///
/// # Returns
/// The database connection
pub fn new(database_url: &str, migration_files: &str) -> Result<Database, String> {
    let database =
        Database::new(database_url).map_err(|e| format!("Error connecting to database: {}", e))?;

    database
        .migrate(migration_files)
        .map_err(|e| format!("Error migrating database: {}", e))?;
    Ok(database)
}
