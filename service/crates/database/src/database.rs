use ::postgres::NoTls;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;

/// Trait to represent our connection to the database
pub trait Database: Send + Sync {
    /// Check out a database client that can be used to query the database
    fn client(&self) -> Option<PooledConnection<PostgresConnectionManager<NoTls>>>;
}
