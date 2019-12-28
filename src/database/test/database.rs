use super::container::TestDatabase;
use crate::database::{migrate::migrate, postgres::PostgresDatabase, Database};
use std::sync::Arc;

/// Wrapper around the database, both the actual Postgres container and the Universe wrapper of it
pub struct TestDatabaseWrapper<'d> {
    #[allow(dead_code)]
    container: TestDatabase<'d>,
    pub wrapper: Arc<dyn Database>,
}

impl<'d> TestDatabaseWrapper<'d> {
    /// Create a new Test Database Wrapper
    pub fn new() -> Self {
        let container = TestDatabase::new();
        let wrapper = Arc::new(PostgresDatabase::new(container.url.clone()).unwrap());

        migrate(wrapper.clone(), "migrations/**/*.sql").unwrap();

        Self { container, wrapper }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_env_log::test;

    #[test]
    fn test_wrapper() {
        TestDatabaseWrapper::new();
    }
}
