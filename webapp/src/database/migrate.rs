/// Trait represnting a database that we're able to migrate to the latest version of the schema
pub trait MigratableDatabase {
    /// Migrate the database to the latest schema version
    ///
    /// # Returns
    /// The result of migrating the database
    fn migrate(&self) -> Result<(), ()>;
}
