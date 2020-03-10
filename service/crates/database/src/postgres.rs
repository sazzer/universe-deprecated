use postgres::NoTls;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use std::sync::Arc;
use tracing::{debug, error};

/// Errors that can be returned when creating a Postgres Database wrapper
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Failed to create connection: {0:?}")]
    ConnectionError(#[from] postgres::Error),
    #[error("Failed to create connection pool: {0:?}")]
    ConnectionPoolError(#[from] r2d2::Error),
}

/// Wrapper around a connection to the Postgres database
#[derive(Debug, Clone)]
pub struct Database {
    pool: Arc<r2d2::Pool<PostgresConnectionManager<NoTls>>>,
}

impl Database {
    /// Create a new wrapper around the postgres database
    ///
    /// # Arguments
    /// * `url` The URL to connect to
    ///
    /// # Returns
    /// The database wrapper, or an error if the URL was bad for some reason
    pub fn new<S>(url: S) -> Result<Database, DatabaseError>
    where
        S: Into<String>,
    {
        let real_url = url.into();
        debug!("Connecting to database: {}", real_url);

        let manager = PostgresConnectionManager::new(real_url.parse()?, NoTls);
        let pool = Pool::new(manager)?;

        Ok(Database {
            pool: Arc::new(pool),
        })
    }

    pub fn client(&self) -> Option<PooledConnection<PostgresConnectionManager<NoTls>>> {
        self.pool
            .get()
            .map_err(|e| {
                error!("Failed to check out connection: {}", e);
                e
            })
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::*;
    use spectral::prelude::*;
    use test_env_log::test;
    use universe_test_database_container::TestDatabase;

    #[test]
    fn test_connect_success() {
        let database = TestDatabase::new();

        let postgres = Database::new(database.url);
        assert_that(&postgres).is_ok();
    }

    #[test]
    fn test_connect_bad_credentials() {
        let database = TestDatabase::new();

        let url = format!(
            "postgres://invalid:invalid@{}:{}",
            database.host, database.port
        );
        let postgres = Database::new(url);
        assert_matches!(postgres.unwrap_err(), DatabaseError::ConnectionPoolError(_));
    }

    #[test]
    fn test_client() {
        let database = TestDatabase::new();

        let postgres = Database::new(database.url).unwrap();
        let mut client = postgres.client().unwrap();
        client.query("SELECT 1", &[]).unwrap();
    }
}
