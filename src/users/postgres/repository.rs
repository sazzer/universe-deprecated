use crate::database::Database;
use crate::entity::Identity;
use crate::users::repository::UserRepository;
use crate::users::{UserData, UserEntity, UserID, Username};
use log::warn;
use postgres::types::ToSql;
use std::sync::Arc;

pub struct PostgresUserRepository {
    #[allow(dead_code)]
    database: Arc<dyn Database>,
}

impl PostgresUserRepository {
    pub fn new(database: Arc<dyn Database>) -> Self {
        Self { database }
    }

    fn get_single_user(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Option<UserEntity> {
        let row = self.database.client().ok()?.query_one(query, params);

        match row {
            Err(e) => {
                warn!("Failed to get user: {}", e);
                None
            }
            Ok(row) => Some(row.into()),
        }
    }
}

impl UserRepository for PostgresUserRepository {
    fn get_user_by_id(&self, user_id: UserID) -> Option<UserEntity> {
        self.get_single_user("SELECT * FROM users WHERE user_id = $1", &[&user_id])
    }
    fn get_user_by_email(&self, email: String) -> Option<UserEntity> {
        self.get_single_user("SELECT * FROM users WHERE email = $1", &[&email])
    }
    fn get_user_by_username(&self, username: Username) -> Option<UserEntity> {
        self.get_single_user("SELECT * FROM users WHERE username = $1", &[&username])
    }
}

impl From<postgres::Row> for UserEntity {
    fn from(row: postgres::Row) -> Self {
        UserEntity {
            identity: Identity {
                id: row.get("user_id"),
                version: row.get("version"),
                created: row.get("created"),
                updated: row.get("updated"),
            },
            data: UserData {
                username: row.get("username"),
                email: row.get("email"),
                display_name: row.get("display_name"),
                password: row.get("password"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabaseWrapper;
    use crate::users::postgres::testdata;
    use spectral::prelude::*;
    use test_env_log::test;
    use uuid::Uuid;
    #[test]
    fn test_find_unknown_user_by_id() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_id(UserID::from_uuid(Uuid::new_v4()));
        assert_that(&user).is_none();
    }
    #[test]
    fn test_find_unknown_user_by_email() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_email("testuser@example.com".to_owned());
        assert_that(&user).is_none();
    }
    #[test]
    fn test_find_unknown_user_by_username() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_username("testuser".parse().unwrap());
        assert_that(&user).is_none();
    }

    #[test]
    fn test_find_known_user_by_id() {
        let testuser = testdata::User::default();
        let database = TestDatabaseWrapper::new();

        database.seed(vec![&testuser]);

        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_id(testuser.user_id.clone());
        assert_user(testuser, user);
    }
    #[test]
    fn test_find_known_user_by_email() {
        let testuser = testdata::User::default();
        let database = TestDatabaseWrapper::new();

        database.seed(vec![&testuser]);

        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_email(testuser.email.clone());
        assert_user(testuser, user);
    }
    #[test]
    fn test_find_known_user_by_username() {
        let testuser = testdata::User::default();
        let database = TestDatabaseWrapper::new();

        database.seed(vec![&testuser]);

        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_username(testuser.username.clone());
        assert_user(testuser, user);
    }

    fn assert_user(testuser: testdata::User, user: Option<UserEntity>) {
        assert_that(&user).is_some().is_equal_to(UserEntity {
            identity: Identity {
                id: testuser.user_id,
                version: testuser.version,
                created: testuser.created,
                updated: testuser.updated,
            },
            data: UserData {
                username: testuser.username,
                email: testuser.email,
                display_name: testuser.display_name,
                password: testuser.password,
            },
        });
    }
}
