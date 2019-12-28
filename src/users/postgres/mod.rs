use crate::database::Database;
use crate::users::repository::UserRepository;
use std::sync::Arc;

pub struct PostgresUserRepository {
    #[allow(dead_code)]
    database: Arc<dyn Database>,
}

impl PostgresUserRepository {
    pub fn new(database: Arc<dyn Database>) -> Self {
        Self { database }
    }
}

impl UserRepository for PostgresUserRepository {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabaseWrapper;
    use test_env_log::test;

    #[test]
    fn test_find_user_by_username() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());
    }
}
