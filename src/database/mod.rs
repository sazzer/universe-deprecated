use postgres::NoTls;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;

pub mod database;

#[cfg(test)]
pub mod test;

/// Possible errors from working with the database
#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    CheckoutError,
}

pub trait Database {
    /// Check out a database client that can be used to query the database
    fn client(&self) -> Result<PooledConnection<PostgresConnectionManager<NoTls>>, DatabaseError>;
}
