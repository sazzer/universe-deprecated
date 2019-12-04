use super::migrate::MigratableDatabase;
use super::{Database, Error};
use log::{error, info};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use universe_migrations::Migrations;

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
    /// TODO: TLS Support
    pub fn new<S: Into<String>>(url: S) -> Result<Self, Error> {
        let manager = PostgresConnectionManager::new(url.into(), TlsMode::None).map_err(|e| {
            error!("Failed to create connection: {:?}", e);
            Error::InstantiationError
        })?;

        let pool = Pool::new(manager).map_err(|e| {
            error!("Failed to create connection pool: {:?}", e);
            Error::InstantiationError
        })?;

        Ok(PostgresDatabase { pool: pool })
    }
}

impl Database<PostgresConnectionManager> for PostgresDatabase {
    fn client(&self) -> Result<PooledConnection<PostgresConnectionManager>, Error> {
        let conn = self.pool.get().map_err(|e| {
            error!("Failed to check out connection: {:?}", e);
            Error::CheckoutError
        })?;

        Ok(conn)
    }
}

impl MigratableDatabase for PostgresDatabase {
    fn migrate<S: Into<String>>(&self, migrations: S) -> Result<u32, String> {
        info!("Migrating database to latest version");

        let count = Migrations::new(self.pool.get().unwrap(), migrations)
            .map_err(|e| format!("Failed to load migrations: {:?}", e))?
            .migrate()
            .map_err(|e| format!("Failed to migrate database: {:?}", e))?;

        Ok(count)
    }
}
