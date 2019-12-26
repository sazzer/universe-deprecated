use super::{Database, DatabaseError};
use log::{debug, error};
use postgres::NoTls;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

/// Errors that can be returned when creating a Postgres Database wrapper
#[derive(Debug, PartialEq)]
pub enum PostgresDatabaseError {
    InstantiationError,
}

/// Wrapper around a connection to the Postgres database
#[derive(Debug)]
pub struct PostgresDatabase {
    pool: r2d2::Pool<PostgresConnectionManager<NoTls>>,
}

impl From<postgres::Error> for PostgresDatabaseError {
    fn from(e: postgres::Error) -> Self {
        error!("Failed to create connection: {:?}", e);
        PostgresDatabaseError::InstantiationError
    }
}

impl From<r2d2::Error> for PostgresDatabaseError {
    fn from(e: r2d2::Error) -> Self {
        error!("Failed to create connection pool: {:?}", e);
        PostgresDatabaseError::InstantiationError
    }
}

impl From<r2d2::Error> for DatabaseError {
    fn from(e: r2d2::Error) -> Self {
        error!("Failed to check out connection: {:?}", e);
        DatabaseError::CheckoutError
    }
}

impl PostgresDatabase {
    /// Create a new wrapper around the postgres database
    ///
    /// # Arguments
    /// * `url` The URL to connect to
    ///
    /// # Returns
    /// The database wrapper, or an error if the URL was bad for some reason
    pub fn new<S>(url: S) -> Result<PostgresDatabase, PostgresDatabaseError>
    where
        S: Into<String>,
    {
        let real_url = url.into();
        debug!("Connecting to database: {}", real_url);

        let manager = PostgresConnectionManager::new(real_url.parse()?, NoTls);
        let pool = Pool::new(manager)?;

        Ok(PostgresDatabase { pool: pool })
    }
}

impl Database for PostgresDatabase {
    fn client(&self) -> Result<PooledConnection<PostgresConnectionManager<NoTls>>, DatabaseError> {
        Ok(self.pool.get()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use crate::database::test::TestDatabase;

    #[test]
    fn test_connect_success() {
        let _ = env_logger::builder().is_test(true).try_init();
        let database = TestDatabase::new();

        let postgres = PostgresDatabase::new(database.url);
        assert_that(&postgres).is_ok();
    }

    #[test]
    fn test_connect_bad_credentials() {
        let _ = env_logger::builder().is_test(true).try_init();
        let database = TestDatabase::new();

        let url = format!("postgres://invalid:invalid@localhost:{}", database.port);
        let postgres = PostgresDatabase::new(url);
        assert_that(&postgres)
            .is_err()
            .is_equal_to(PostgresDatabaseError::InstantiationError);
    }

    #[test]
    fn test_client() {
        let _ = env_logger::builder().is_test(true).try_init();
        let database = TestDatabase::new();

        let postgres = PostgresDatabase::new(database.url).unwrap();
        let mut client = postgres.client().unwrap();
        client.query("SELECT 1", &[]).unwrap();
    }
}
