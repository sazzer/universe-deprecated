use std::sync::Arc;
use universe_database::Database;
use universe_test_database_container::TestDatabase;

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
        let migrations_base = std::fs::canonicalize("../../migrations").unwrap();

        let container = TestDatabase::new();
        let wrapper = universe_database::builder::new(
            container.url.clone(),
            format!("{}/**/*.sql", migrations_base.to_str().unwrap()),
        )
        .unwrap();

        Self { container, wrapper }
    }

    pub fn url(&self) -> String {
        self.container.url.clone()
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
