use super::migrate::MigratableDatabase;
use super::{Database, Error};
use log::{error, info};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use universe_migrations::{Migrations, MigrationsError};

/// Wrapper around the database connection
pub struct PostgresDatabase {
    pool: r2d2::Pool<PostgresConnectionManager>,
}

impl From<postgres::Error> for Error {
    fn from(e: postgres::Error) -> Error {
        error!("Failed to connect to database: {:?}", e);
        let message = format!("Failed to connect to database: {:?}", e);

        Error(message)
    }
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Error {
        error!("Database error: {:?}", e);
        let message = format!("Database error: {:?}", e);

        Error(message)
    }
}

impl From<MigrationsError<'_>> for Error {
    fn from(e: MigrationsError) -> Error {
        error!("Migrations error: {:?}", e);
        let message = format!("Migrations error: {:?}", e);

        Error(message)
    }
}

impl PostgresDatabase {
    /// Construct a new database connection to the database specified in the provided URL.
    ///
    /// # Arguments
    /// * `url` The URL to connect to
    ///
    /// # Returns
    /// The database connection wrapper
    ///
    /// # Todo
    /// TODO: TLS Support
    pub fn new<S: Into<String>>(url: S) -> Result<Self, Error> {
        let manager = PostgresConnectionManager::new(url.into(), TlsMode::None)?;
        let pool = Pool::new(manager)?;

        Ok(PostgresDatabase { pool: pool })
    }
}

impl Database<PostgresConnectionManager> for PostgresDatabase {
    fn client(&self) -> Result<PooledConnection<PostgresConnectionManager>, Error> {
        let conn = self.pool.get()?;

        Ok(conn)
    }
}

impl MigratableDatabase for PostgresDatabase {
    fn migrate<S: Into<String>>(&self, migrations: S) -> Result<u32, Error> {
        info!("Migrating database to latest version");

        let count = Migrations::new(self.pool.get()?, migrations)?.migrate()?;

        Ok(count)
    }
}
