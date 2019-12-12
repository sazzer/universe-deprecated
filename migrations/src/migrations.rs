use glob::glob;
use log::{error, info, warn};
use r2d2::{ManageConnection, PooledConnection};
use std::borrow::Cow;
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
pub enum MigrationsError<'a> {
    IoError(Cow<'a, str>),
    DatabaseError(Cow<'a, str>),
}

impl From<std::io::Error> for MigrationsError<'_> {
    fn from(err: std::io::Error) -> Self {
        error!("IO Error: {:?}", err);
        MigrationsError::IoError(err.to_string().into())
    }
}

impl From<glob::PatternError> for MigrationsError<'_> {
    fn from(err: glob::PatternError) -> Self {
        error!("Error listing files: {:?}", err);
        MigrationsError::IoError(err.to_string().into())
    }
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
    pub fn new<'a, S: Into<String>>(
        conn: PooledConnection<M>,
        migrations_dir: S,
    ) -> Result<Self, MigrationsError<'a>> {
        let glob_path = format!("{}/**/*.sql", migrations_dir.into());
        let mut files: Vec<PathBuf> = glob(&glob_path)?.filter_map(Result::ok).collect();
        files.sort();

        if files.is_empty() {
            warn!("No migrations found to apply in directory: {}", glob_path);
        } else {
            info!("Migrations to apply: {:?}", files);
        }

        Ok(Self {
            files: files,
            conn: conn,
        })
    }
}
