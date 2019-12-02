use super::migrate::MigratableDatabase;
use super::Database;
use log::info;
use r2d2::{Error, Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

/// Wrapper around the database connection
pub struct PostgresDatabase {
    pool: r2d2::Pool<PostgresConnectionManager>,
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
    /// TODO: Error Handling
    /// TODO: TLS Support
    pub fn new<S: Into<String>>(url: S) -> Self {
        let manager = PostgresConnectionManager::new(url.into(), TlsMode::None).unwrap();

        let pool = Pool::new(manager).unwrap();

        PostgresDatabase { pool: pool }
    }
}

impl Database<PostgresConnectionManager> for PostgresDatabase {
    fn client(&self) -> Result<PooledConnection<PostgresConnectionManager>, Error> {
        self.pool.get()
    }
}

impl MigratableDatabase for PostgresDatabase {
    fn migrate(&self) -> Result<(), ()> {
        info!("Migrating database to latest version");
        Ok(())
    }
}
