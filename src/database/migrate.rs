use super::Database;
use log::{debug, error, info};
use postgres::Transaction;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

/// Error returned when migrating the database fails for some reason
#[derive(Debug, PartialEq)]
pub struct MigrationError {
    message: String,
}

/// Migrate the database to the latest schema version as described by the files in the given directory
///
/// # Arguments
/// * `database` The database to migrate
/// * `migrations_dir` The migration files to apply
///
/// # Returns
/// If an error occurred then the error is returned. If not then no return value
pub fn migrate<S>(database: Arc<dyn Database>, migrations_dir: S) -> Result<(), MigrationError>
where
    S: Into<String>,
{
    let files = list_migration_files(migrations_dir.into())?;
    info!("Migrations to apply: {:?}", files);

    if files.len() > 0 {
        let mut client = database.client()?;
        let mut transaction = client.transaction()?;

        ensure_migrations_table(&mut transaction)?;
        apply_migrations(files, &mut transaction)?;

        transaction.commit()?;
    }
    Ok(())
}

/// Generate a list of the migration files that we want to apply
///
/// # Arguments
/// * `migrations_dir` The migration files to apply
///
/// # Returns
/// The list of files, in order, that we want to apply
fn list_migration_files(migrations_dir: String) -> Result<Vec<PathBuf>, MigrationError> {
    let mut files: Vec<PathBuf> = fs::read_dir(migrations_dir)?
        .filter_map(|res| res.ok())
        .filter(|res| res.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|res| res.path())
        .filter(|res| res.extension().and_then(OsStr::to_str) == Some("sql"))
        .collect();

    files.sort();

    Ok(files)
}

/// Ensure that the migrations table exists in the database and that it is locked for only us to use.
/// Locking the table helps to ensure that only one migrations task is running at a time.
///
/// # Arguments
/// * `transaction` The transaction to use to create and lock the table
///
/// # Returns
/// If an error occurred then the error is returned. If not then no return value
fn ensure_migrations_table(transaction: &mut Transaction) -> Result<(), MigrationError> {
    transaction.execute(
        "CREATE TABLE IF NOT EXISTS __migrations(
                migration_file TEXT PRIMARY KEY,
                sequence SERIAL NOT NULL,
                executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                executed_from TEXT NOT NULL DEFAULT inet_client_addr()
            )",
        &[],
    )?;
    transaction.execute("LOCK TABLE __migrations IN EXCLUSIVE MODE", &[])?;

    Ok(())
}

/// Generate a list of all the migrations that have previously been applied to this database.
///
/// # Arguments
/// * `transaction` The transaction to use to list the migrations that were previously applied
///
/// # Returns
/// The list of migrations that have already been applied
fn list_applied_migrations(transaction: &mut Transaction) -> Result<Vec<String>, MigrationError> {
    let migrations = transaction
        .query("SELECT migration_file FROM __migrations", &[])?
        .iter()
        .map(|row| row.get::<&str, String>("migration_file"))
        .collect::<Vec<String>>();
    info!("Migrations already applied: {:?}", migrations);

    Ok(migrations)
}

/// Attempt to apply all of the migrations from the provided list of files
///
/// # Arguments
/// * `files` The files containing migrations to apply
/// * `transaction` The transaction within which the migrations should be applied
///
/// # Returns
/// The number of migrations that were actually applied
fn apply_migrations(
    files: Vec<PathBuf>,
    transaction: &mut Transaction,
) -> Result<u32, MigrationError> {
    let applied_migrations: Vec<String> = list_applied_migrations(transaction)?;

    let mut applied = 0;
    for entry in files.iter() {
        if applied_migrations.contains(&entry.to_str().unwrap().to_owned()) {
            debug!("Already processed file: {:?}", entry);
        } else {
            debug!("Processing file: {:?}", entry);
            let source: String = fs::read_to_string(&entry)?;

            transaction.query(source.as_str(), &[])?;

            transaction.execute(
                "INSERT INTO __migrations(migration_file) VALUES ($1)",
                &[&entry.to_str()],
            )?;
            applied += 1;
        }
    }
    debug!("Applied {} out of {} migrations", applied, files.len());

    Ok(applied)
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MigrationError {}

impl From<std::io::Error> for MigrationError {
    fn from(e: std::io::Error) -> Self {
        let message = format!("IO Error performing database migration: {}", e);
        error!("{}", message);
        MigrationError { message }
    }
}

impl From<super::DatabaseError> for MigrationError {
    fn from(e: super::DatabaseError) -> Self {
        let message = format!("Database Error performing database migration: {}", e);
        error!("{}", message);
        MigrationError { message }
    }
}

impl From<postgres::Error> for MigrationError {
    fn from(e: postgres::Error) -> Self {
        let message = format!("Database Error performing database migration: {}", e);
        error!("{}", message);
        MigrationError { message }
    }
}
