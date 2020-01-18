use crate::service::repository::UserRepository;
use crate::{UserEntity, UserID, Username};
use postgres::types::ToSql;
use std::sync::Arc;
use tracing::warn;
use universe_database::Database;

/// User Repository that works in terms of the Postgres database
pub struct PostgresUserRepository {
    database: Arc<dyn Database>,
}

impl PostgresUserRepository {
    /// Construct a new Postgres User repository
    ///
    /// # Arguments
    /// * `database` The database connection to use
    ///
    /// # Returns
    /// The User Repository
    pub fn new(database: Arc<dyn Database>) -> Self {
        Self { database }
    }

    /// Helper to get a single user record using the given query and binds.
    fn get_single_user(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Option<UserEntity> {
        self.database
            .client()?
            .query_one(query, params)
            .map_err(|e| {
                warn!("Failed to get user: {}", e);
                e
            })
            .ok()
            .map(|row| row.into())
    }
}

impl UserRepository for PostgresUserRepository {
    fn get_user_by_id(&self, user_id: &UserID) -> Option<UserEntity> {
        self.get_single_user("SELECT * FROM users WHERE user_id = $1", &[user_id])
    }
    fn get_user_by_email(&self, email: &str) -> Option<UserEntity> {
        self.get_single_user("SELECT * FROM users WHERE email = $1", &[&email])
    }
    fn get_user_by_username(&self, username: &Username) -> Option<UserEntity> {
        self.get_single_user("SELECT * FROM users WHERE username = $1", &[username])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use universe_test_database_wrapper::TestDatabaseWrapper;
    use universe_testdata::User;
    use uuid::Uuid;

    #[test]
    fn test_find_unknown_user_by_id() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_id(&UserID::from_uuid(Uuid::new_v4()));
        assert_that(&user).is_none();
    }

    #[test]
    fn test_find_unknown_user_by_email() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_email(&"testuser@example.com".to_owned());
        assert_that(&user).is_none();
    }

    #[test]
    fn test_find_unknown_user_by_username() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_username(&"testuser".parse().unwrap());
        assert_that(&user).is_none();
    }

    #[test]
    fn test_find_known_user_by_id() {
        let testuser = User::default();
        let database = TestDatabaseWrapper::new();

        database.seed(vec![&testuser]);

        let repository = PostgresUserRepository::new(database.wrapper.clone());
        let user_id = UserID::from_uuid(testuser.user_id.clone());
        let user = repository.get_user_by_id(&user_id);
        assert_user(testuser, user);
    }

    #[test]
    fn test_find_known_user_by_email() {
        let testuser = User::default();
        let database = TestDatabaseWrapper::new();

        database.seed(vec![&testuser]);

        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_email(&testuser.email.clone());
        assert_user(testuser, user);
    }

    #[test]
    fn test_find_known_user_by_username() {
        let testuser = User::default();
        let database = TestDatabaseWrapper::new();

        database.seed(vec![&testuser]);

        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_username(&testuser.username.parse().unwrap());
        assert_user(testuser, user);
    }

    fn assert_user(testuser: User, user: Option<UserEntity>) {
        assert_that(&user)
            .is_some()
            .is_equal_to(UserEntity::from(testuser));
    }
}
