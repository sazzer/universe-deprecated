use ::postgres::NoTls;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;

pub mod migrate;
pub mod postgres;

#[cfg(test)]
pub mod test;

pub trait Database {
    /// Check out a database client that can be used to query the database
    fn client(&self) -> Option<PooledConnection<PostgresConnectionManager<NoTls>>>;
}
