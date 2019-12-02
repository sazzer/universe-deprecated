use r2d2::{Error, ManageConnection, PooledConnection};

/// Trait representing everything we can do with a database connection
pub trait Database<M>
where
    M: ManageConnection,
{
    fn client(&self) -> Result<PooledConnection<M>, Error>;
}
