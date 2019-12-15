use r2d2::{ManageConnection, PooledConnection};

#[derive(Debug, PartialEq)]
pub struct Error(pub String);

/// Trait representing everything we can do with a database connection
pub trait Database<M>: Send + Sync
where
    M: ManageConnection,
{
    /// Check out a new database connection to use
    ///
    /// # Returns
    /// The database connection to use
    fn client(&self) -> Result<PooledConnection<M>, Error>;
}
