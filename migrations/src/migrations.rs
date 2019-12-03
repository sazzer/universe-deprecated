use log::info;
use r2d2::{ManageConnection, PooledConnection};
use std::fs;
use std::path::PathBuf;

/// Struct that represents the ability to migrate a database to the latest version of the schema
pub struct Migrations<M>
where
    M: ManageConnection,
{
    pub(crate) files: Vec<PathBuf>,
    pub(crate) conn: PooledConnection<M>,
}

/// Possible errors that can occur from the migrations
#[derive(Debug, PartialEq)]
pub enum MigrationsError {
    UnknownDirectory,
    DatabaseError(String),
    MigrationError(String),
}

impl<M> Migrations<M>
where
    M: ManageConnection,
{
    /// Construct the new Migrations instance ready to run.
    ///
    /// Migrations are executed from the provided directory in strict filename order, and only
    /// if they are not already recorded as having been executed on this database before.
    ///
    /// # Arguments
    /// * `conn` The R2D2 Connection to use to perform the migration
    /// * `migrations_dir` The directory containing all the SQL scripts to execute.
    ///
    /// # Returns
    /// the migrations instance to actually use to run the migrations
    pub fn new<S: Into<String>>(
        conn: PooledConnection<M>,
        migrations_dir: S,
    ) -> Result<Self, MigrationsError> {
        let mut files: Vec<PathBuf> = fs::read_dir(migrations_dir.into())
            .map_err(|_| MigrationsError::UnknownDirectory)?
            .filter(|res| res.as_ref().unwrap().file_type().unwrap().is_file())
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();
        files.sort();
        info!("Migrations to apply: {:?}", files);

        Ok(Self {
            files: files,
            conn: conn,
        })
    }
}
