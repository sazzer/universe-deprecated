use super::Database;
use glob::glob;
use postgres::Transaction;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error, info};

pub trait MigratableDatabase {
    /// Migrate the database to the latest schema version as described by the files in the given directory
    ///
    /// # Arguments
    /// * `migrations` The migration files to apply
    ///
    /// # Returns
    /// If an error occurred then the error is returned. If not then no return value
    fn migrate<S>(&self, migrations: S) -> Result<u32, MigrationError>
    where
        S: Into<String>;
}

impl MigratableDatabase for Database {
    fn migrate<S>(&self, migrations: S) -> Result<u32, MigrationError>
    where
        S: Into<String>,
    {
        let files = list_migration_files(migrations.into())?;
        info!("Migrations to apply: {:?}", files);

        let mut applied = 0;
        if !files.is_empty() {
            let mut client = self.client().ok_or(MigrationError::ConnectionError)?;
            let mut transaction = client.transaction()?;

            ensure_migrations_table(&mut transaction)?;
            applied = apply_migrations(files, &mut transaction)?;

            transaction.commit()?;
        }
        Ok(applied)
    }
}

/// Generate a list of the migration files that we want to apply
///
/// # Arguments
/// * `migrations` The migration files to apply
///
/// # Returns
/// The list of files, in order, that we want to apply
fn list_migration_files(migrations: String) -> Result<Vec<PathBuf>, MigrationError> {
    info!("Loading migrations from: {:?}", migrations);

    let mut files: Vec<PathBuf> = glob(&migrations)?.filter_map(|res| res.ok()).collect();
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
        let filename = entry
            .file_name()
            .map(|f| f.to_str().unwrap())
            .map(|f| f.to_owned())
            .unwrap();

        if applied_migrations.contains(&filename) {
            debug!("Already processed file: {:?}", entry);
        } else {
            debug!("Processing file: {:?}", entry);
            let source: String = fs::read_to_string(&entry)?;
            let commands = source.split(";\n");

            for command in commands {
                transaction.query(command, &[])?;
            }

            transaction.execute(
                "INSERT INTO __migrations(migration_file) VALUES ($1)",
                &[&filename],
            )?;
            applied += 1;
        }
    }
    debug!("Applied {} out of {} migrations", applied, files.len());

    Ok(applied)
}

/// Error returned when migrating the database fails for some reason
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("IO Error performing database migration: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Glob Error loading migrations: {0}")]
    GlobError(#[from] glob::PatternError),
    #[error("Database Error performing database migration: {0}")]
    QueryError(#[from] postgres::Error),
    #[error("Error getting database connection")]
    ConnectionError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Database;
    use assert_matches::*;
    use spectral::prelude::*;
    use test_env_log::test;
    use universe_test_database_container::TestDatabase;

    #[test]
    fn test_invalid_migrations_glob() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("****");

        assert_matches!(result.unwrap_err(), MigrationError::GlobError(_));

        let tables = wrapper
                        .client().unwrap()
                        .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap();

        assert_that(&tables).is_empty();
    }

    #[test]
    fn test_no_migrations_directory() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("test_migrations/missing/**/*.sql");

        assert_that(&result).is_ok_containing(0);
        let tables = wrapper
                        .client().unwrap()
                        .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap();

        assert_that(&tables).is_empty();
    }

    #[test]
    fn test_no_migrations() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("test_migrations/empty/**/*.sql");

        assert_that(&result).is_ok_containing(0);
        let tables: Vec<String> = wrapper
                            .client().unwrap()
                            .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap()
                            .into_iter()
                            .map(|row| row.get::<&str, String>("table_name"))
                            .collect();

        assert_that(&tables).is_empty();
    }

    #[test]
    fn test_some_migrations() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("test_migrations/full/**/*.sql");

        assert_that(&result).is_ok_containing(2);
        let tables: Vec<String> = wrapper
                                .client().unwrap()
                                .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap()
                                .into_iter()
                                .map(|row| row.get::<&str, String>("table_name"))
                                .collect();

        assert_that(&tables).has_length(3);
        assert_that(&tables).contains("__migrations".to_owned());
        assert_that(&tables).contains("first".to_owned());
        assert_that(&tables).contains("second".to_owned());

        let migrations: Vec<String> = wrapper
            .client()
            .unwrap()
            .query(
                "SELECT migration_file FROM __migrations ORDER BY sequence ASC",
                &[],
            )
            .unwrap()
            .into_iter()
            .map(|row| row.get::<&str, String>("migration_file"))
            .collect();

        assert_that(&migrations).is_equal_to(vec![
            "00001-first.sql".to_owned(),
            "00002-second.sql".to_owned(),
        ]);
    }

    #[test]
    fn test_some_migrations_again() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("test_migrations/full/**/*.sql");

        assert_that(&result).is_ok_containing(2);

        let result2 = wrapper.migrate("test_migrations/full/**/*.sql");

        assert_that(&result2).is_ok_containing(0);

        let tables: Vec<String> = wrapper
                                    .client().unwrap()
                                    .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap()
                                    .into_iter()
                                    .map(|row| row.get::<&str, String>("table_name"))
                                    .collect();

        assert_that(&tables).has_length(3);
        assert_that(&tables).contains("__migrations".to_owned());
        assert_that(&tables).contains("first".to_owned());
        assert_that(&tables).contains("second".to_owned());

        let migrations: Vec<String> = wrapper
            .client()
            .unwrap()
            .query(
                "SELECT migration_file FROM __migrations ORDER BY sequence ASC",
                &[],
            )
            .unwrap()
            .into_iter()
            .map(|row| row.get::<&str, String>("migration_file"))
            .collect();

        assert_that(&migrations).is_equal_to(vec![
            "00001-first.sql".to_owned(),
            "00002-second.sql".to_owned(),
        ]);
    }

    #[test]
    fn test_additional_migrations() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("test_migrations/full/00001-first.sql");

        assert_that(&result).is_ok_containing(1);

        let tables: Vec<String> = wrapper
                                            .client().unwrap()
                                            .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap()
                                            .into_iter()
                                            .map(|row| row.get::<&str, String>("table_name"))
                                            .collect();

        assert_that(&tables).has_length(2);
        assert_that(&tables).contains("__migrations".to_owned());
        assert_that(&tables).contains("first".to_owned());

        let migrations: Vec<String> = wrapper
            .client()
            .unwrap()
            .query(
                "SELECT migration_file FROM __migrations ORDER BY sequence ASC",
                &[],
            )
            .unwrap()
            .into_iter()
            .map(|row| row.get::<&str, String>("migration_file"))
            .collect();

        assert_that(&migrations).is_equal_to(vec!["00001-first.sql".to_owned()]);

        // Now run the rest of the files
        let result2 = wrapper.migrate("test_migrations/full/**/*.sql");

        assert_that(&result2).is_ok_containing(1);

        let tables: Vec<String> = wrapper
                                    .client().unwrap()
                                    .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap()
                                    .into_iter()
                                    .map(|row| row.get::<&str, String>("table_name"))
                                    .collect();

        assert_that(&tables).has_length(3);
        assert_that(&tables).contains("__migrations".to_owned());
        assert_that(&tables).contains("first".to_owned());
        assert_that(&tables).contains("second".to_owned());

        let migrations: Vec<String> = wrapper
            .client()
            .unwrap()
            .query(
                "SELECT migration_file FROM __migrations ORDER BY sequence ASC",
                &[],
            )
            .unwrap()
            .into_iter()
            .map(|row| row.get::<&str, String>("migration_file"))
            .collect();

        assert_that(&migrations).is_equal_to(vec![
            "00001-first.sql".to_owned(),
            "00002-second.sql".to_owned(),
        ]);
    }

    #[test]
    fn test_invalid_migrations() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("test_migrations/invalid/**/*.sql");

        assert_matches!(result.unwrap_err(), MigrationError::QueryError(_));

        let tables = wrapper
                            .client().unwrap()
                            .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap();

        assert_that(&tables).is_empty();
    }

    #[test]
    fn test_multiple_migrations() {
        let database = TestDatabase::new();
        let wrapper = Database::new(database.url).unwrap();

        let result = wrapper.migrate("test_migrations/multiple/**/*.sql");

        assert_that(&result).is_ok_containing(1);
        let tables: Vec<String> = wrapper
                                .client().unwrap()
                                .query("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'postgres' AND table_schema = 'public'", &[]).unwrap()
                                .into_iter()
                                .map(|row| row.get::<&str, String>("table_name"))
                                .collect();

        assert_that(&tables).has_length(3);
        assert_that(&tables).contains("__migrations".to_owned());
        assert_that(&tables).contains("third_one".to_owned());
        assert_that(&tables).contains("third_two".to_owned());

        let migrations: Vec<String> = wrapper
            .client()
            .unwrap()
            .query(
                "SELECT migration_file FROM __migrations ORDER BY sequence ASC",
                &[],
            )
            .unwrap()
            .into_iter()
            .map(|row| row.get::<&str, String>("migration_file"))
            .collect();

        assert_that(&migrations).is_equal_to(vec!["00001-multiple.sql".to_owned()]);
    }
}
