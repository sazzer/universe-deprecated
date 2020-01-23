use std::ops::Deref;
use universe_database::Database;
use universe_test_database_container::TestDatabase;

/// Wrapper around the database, both the actual Postgres container and the Universe wrapper of it
pub struct TestDatabaseWrapper<'d> {
    #[allow(dead_code)]
    container: TestDatabase<'d>,
    pub wrapper: Database,
}

impl<'d> Default for TestDatabaseWrapper<'d> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'d> TestDatabaseWrapper<'d> {
    /// Create a new Test Database Wrapper
    pub fn new() -> Self {
        let migrations_base = std::fs::canonicalize("../../migrations").unwrap();
        let migrations_glob = format!("{}/**/*.sql", migrations_base.to_str().unwrap());

        let container = TestDatabase::new();
        let wrapper = universe_database::builder::new(&container.url, &migrations_glob).unwrap();

        Self { container, wrapper }
    }

    pub fn url(&self) -> String {
        self.container.url.clone()
    }
}

impl<'d> Deref for TestDatabaseWrapper<'d> {
    type Target = Database;
    fn deref(&self) -> &Database {
        &self.wrapper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper() {
        TestDatabaseWrapper::new();
    }
}
