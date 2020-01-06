use super::container::TestDatabase;
use crate::database::{migrate::migrate, postgres::PostgresDatabase, Database};
use std::sync::Arc;

/// Wrapper around the database, both the actual Postgres container and the Universe wrapper of it
pub struct TestDatabaseWrapper<'d> {
    #[allow(dead_code)]
    container: TestDatabase<'d>,
    pub wrapper: Arc<dyn Database>,
}

pub trait TestData {
    /// Generate the SQL needed to insert this test data into the database
    fn sql(&self) -> String;

    /// Generate the binds that go with the SQL
    fn binds(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)>;
}

impl<'d> TestDatabaseWrapper<'d> {
    /// Create a new Test Database Wrapper
    pub fn new() -> Self {
        let container = TestDatabase::new();
        let wrapper = Arc::new(PostgresDatabase::new(container.url.clone()).unwrap());

        migrate(wrapper.clone(), "migrations/**/*.sql").unwrap();

        Self { container, wrapper }
    }

    /// Seed the database with the provided test data, all inserted in the provided order and in the
    /// same database transaction
    pub fn seed(&self, data: Vec<&dyn TestData>) {
        let mut client = self.wrapper.client().unwrap();
        let mut transaction = client.transaction().unwrap();

        for d in data.iter() {
            transaction
                .query(d.sql().as_str(), &(d.binds()[..]))
                .unwrap();
        }

        transaction.commit().unwrap();
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
