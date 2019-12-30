use crate::database::Database;
use crate::entity::Identity;
use crate::users::repository::UserRepository;
use crate::users::{UserData, UserEntity, Username};
use log::warn;
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

impl UserRepository for PostgresUserRepository {
    fn get_user_by_username(&self, username: Username) -> Option<UserEntity> {
        let row = self
            .database
            .client()
            .ok()?
            .query_one("SELECT * FROM users WHERE username = $1", &[&username]);

        match row {
            Err(e) => {
                warn!("Failed to get user: {}", e);
                None
            }
            Ok(row) => Some(row.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabaseWrapper;
    use spectral::prelude::*;
    use test_env_log::test;
    #[test]
    fn test_find_unknown_user_by_username() {
        let database = TestDatabaseWrapper::new();
        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_username("testuser".parse().unwrap());
        assert_that(&user).is_none();
    }

    #[test]
    fn test_find_known_user_by_username() {
        let testuser = crate::users::postgres::testdata::User::default();
        let database = TestDatabaseWrapper::new();

        database.seed(vec![&testuser]);

        let repository = PostgresUserRepository::new(database.wrapper.clone());

        let user = repository.get_user_by_username("testuser".parse().unwrap());
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
