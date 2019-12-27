use ::postgres::NoTls;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;

pub mod migrate;
pub mod postgres;

#[cfg(test)]
pub mod test;

/// Possible errors from working with the database
#[derive(Debug, PartialEq)]
pub struct DatabaseError {
    message: String,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DatabaseError {}

pub trait Database {
    /// Check out a database client that can be used to query the database
    fn client(&self) -> Result<PooledConnection<PostgresConnectionManager<NoTls>>, DatabaseError>;
}
