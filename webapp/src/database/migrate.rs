use super::Error;

/// Trait represnting a database that we're able to migrate to the latest version of the schema
pub trait MigratableDatabase {
    /// Migrate the database to the latest schema version
    ///
    /// # Arguments
    /// # `migrations` The directory containing the migrations
    ///
    /// # Returns
    /// The result of migrating the database
    fn migrate<S: Into<String>>(&self, migrations: S) -> Result<u32, Error>;
}
