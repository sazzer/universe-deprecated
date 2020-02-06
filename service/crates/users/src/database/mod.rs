use crate::model::*;
use crate::service::repository::*;
use std::error::Error;
use tracing::{debug, warn};
use universe_database::Database;
use universe_entity::Identity;

impl From<&postgres::Row> for UserEntity {
    fn from(row: &postgres::Row) -> Self {
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

impl UserRepository for Database {
    fn get_user_by_id(&self, user_id: &UserID) -> Option<UserEntity> {
        let mut client = self.client().unwrap();

        let user = client
            .query("SELECT * FROM users WHERE user_id = $1", &[&user_id])
            .map_err(|e| {
                warn!("Error loading user from database: {}", e);
                e
            })
            .ok()
            .filter(|rows| rows.len() > 0)
            .and_then(|rows| rows.get(0).map(|row| row.into()));

        debug!("User for ID {}: {:?}", user_id, user);
        user
    }

    fn get_user_by_username(&self, username: &Username) -> Option<UserEntity> {
        let mut client = self.client().unwrap();

        let user = client
            .query(
                "SELECT * FROM users WHERE UPPER(username) = UPPER($1)",
                &[&username],
            )
            .map_err(|e| {
                warn!("Error loading user from database: {}", e);
                e
            })
            .ok()
            .filter(|rows| rows.len() > 0)
            .and_then(|rows| rows.get(0).map(|row| row.into()));

        debug!("User for username {}: {:?}", username, user);
        user
    }

    fn create_user(&self, user: UserEntity) -> Result<UserEntity, PersistUserError> {
        debug!("Creating record for user: {:?}", user);

        let mut client = self.client().unwrap();

        let result = client.query("INSERT INTO users(user_id, version, created, updated, username, email, display_name, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *", &[
            &user.identity.id,
            &user.identity.version,
            &user.identity.created,
            &user.identity.updated,
            &user.data.username,
            &user.data.email,
            &user.data.display_name,
            &user.data.password,
        ])
        .map(|rows| rows.get(0).unwrap().into())?;

        debug!("Created record for user: {:?}", result);

        Ok(result)
    }
}

impl From<postgres::Error> for PersistUserError {
    fn from(error: postgres::Error) -> Self {
        warn!("Error creating user in database: {:?}", error);

        error
            .source()
            .and_then(|e| e.downcast_ref::<postgres::error::DbError>())
            .map(|e| match e.constraint() {
                Some("users_pkey") => PersistUserError::DuplicateId,
                Some("users_username_key") => PersistUserError::DuplicateUsername,
                Some("users_email_key") => PersistUserError::DuplicateEmail,
                _ => PersistUserError::UnknownError,
            })
            .unwrap_or(PersistUserError::UnknownError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use universe_test_database_wrapper::TestDatabaseWrapper;
    use universe_testdata::{seed, User};

    #[test]
    fn test_get_unknown_user_by_id() {
        let database = TestDatabaseWrapper::new();
        let user_id: UserID = Default::default();

        let user = database.wrapper.get_user_by_id(&user_id);
        assert_that(&user).is_none();
    }

    #[test]
    fn test_get_known_user_by_id() {
        let database = TestDatabaseWrapper::new();
        let seeded_user: User = Default::default();
        seed(&database, vec![&seeded_user]);

        let user_id = UserID::from_uuid(seeded_user.user_id);
        let user = database.wrapper.get_user_by_id(&user_id);
        assert_that(&user).is_some();

        let user = user.unwrap();
        assert_that(&user).is_equal_to(UserEntity::from(seeded_user));
    }

    #[test]
    fn test_get_unknown_user_by_username() {
        let database = TestDatabaseWrapper::new();
        let username: Username = "testuser".parse().unwrap();

        let user = database.wrapper.get_user_by_username(&username);
        assert_that(&user).is_none();
    }

    #[test]
    fn test_get_known_user_by_username() {
        let database = TestDatabaseWrapper::new();
        let seeded_user: User = Default::default();
        seed(&database, vec![&seeded_user]);

        let username: Username = seeded_user.username.parse().unwrap();
        let user = database.wrapper.get_user_by_username(&username);
        assert_that(&user).is_some();

        let user = user.unwrap();
        assert_that(&user).is_equal_to(UserEntity::from(seeded_user));
    }

    #[test]
    fn test_get_known_user_by_username_different_case() {
        let database = TestDatabaseWrapper::new();
        let seeded_user: User = User {
            username: "TestUser".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&seeded_user]);

        let username: Username = "testuser".parse().unwrap();
        let user = database.wrapper.get_user_by_username(&username);
        assert_that(&user).is_some();

        let user = user.unwrap();
        assert_that(&user).is_equal_to(UserEntity::from(seeded_user));
    }

    #[test]
    fn test_create_user() {
        let database = TestDatabaseWrapper::new();
        let seeded_user: User = Default::default();

        let created_user = database
            .wrapper
            .create_user(seeded_user.clone().into())
            .unwrap();

        assert_that(&created_user).is_equal_to(UserEntity::from(seeded_user));

        let loaded_user = database
            .wrapper
            .get_user_by_id(&created_user.identity.id)
            .unwrap();
        assert_that(&loaded_user).is_equal_to(created_user);
    }

    #[test]
    fn test_create_user_duplicate_id() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let seeded_user: User = User {
            user_id: existing_user.user_id,
            username: "new_username".to_owned(),
            email: "new@example.com".to_owned(),
            ..Default::default()
        };

        let created_user = database
            .wrapper
            .create_user(seeded_user.clone().into())
            .unwrap_err();

        assert_that(&created_user).is_equal_to(PersistUserError::DuplicateId);
    }

    #[test]
    fn test_create_user_duplicate_username() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let seeded_user: User = User {
            username: "testuser".to_owned(),
            email: "new@example.com".to_owned(),
            ..Default::default()
        };

        let created_user = database
            .wrapper
            .create_user(seeded_user.clone().into())
            .unwrap_err();

        assert_that(&created_user).is_equal_to(PersistUserError::DuplicateUsername);
    }

    #[test]
    fn test_create_user_duplicate_username_different_case() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let seeded_user: User = User {
            username: "TestUser".to_owned(),
            email: "new@example.com".to_owned(),
            ..Default::default()
        };

        let created_user = database
            .wrapper
            .create_user(seeded_user.clone().into())
            .unwrap_err();

        assert_that(&created_user).is_equal_to(PersistUserError::DuplicateUsername);
    }

    #[test]
    fn test_create_user_duplicate_email() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let seeded_user: User = User {
            username: "new_username".to_owned(),
            email: "testuser@example.com".to_owned(),
            ..Default::default()
        };

        let created_user = database
            .wrapper
            .create_user(seeded_user.clone().into())
            .unwrap_err();

        assert_that(&created_user).is_equal_to(PersistUserError::DuplicateEmail);
    }

    #[test]
    fn test_create_user_duplicate_email_different_case() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let seeded_user: User = User {
            username: "new_username".to_owned(),
            email: "TestUser@example.com".to_owned(),
            ..Default::default()
        };

        let created_user = database
            .wrapper
            .create_user(seeded_user.clone().into())
            .unwrap_err();

        assert_that(&created_user).is_equal_to(PersistUserError::DuplicateEmail);
    }
}
