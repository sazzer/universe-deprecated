use crate::model::*;
use crate::service::repository::*;
use chrono::Utc;
use std::error::Error;
use tracing::{debug, warn};
use universe_database::Database;
use universe_entity::Identity;
use uuid::Uuid;

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
            .filter(|rows| !rows.is_empty())
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
            .filter(|rows| !rows.is_empty())
            .and_then(|rows| rows.get(0).map(|row| row.into()));

        debug!("User for username {}: {:?}", username, user);
        user
    }

    fn create_user(&self, user: UserData) -> Result<UserEntity, PersistUserError> {
        debug!("Creating record for user: {:?}", user);

        let mut client = self.client().unwrap();

        let new_id = UserID::default();
        let new_version = Uuid::new_v4();
        let new_updated = Utc::now();

        let result = client.query(
            "INSERT INTO users(user_id, version, created, updated, username, email, display_name, password) 
                VALUES ($1, $2, $3, $3, $4, $5, $6, $7) 
                RETURNING *", &[
            &new_id,
            &new_version,
            &new_updated,
            &user.username,
            &user.email,
            &user.display_name,
            &user.password,
        ])
        .map(|rows| rows.get(0).unwrap().into())?;

        debug!("Created record for user: {:?}", result);

        Ok(result)
    }

    fn update_user(&self, user: UserEntity) -> Result<UserEntity, PersistUserError> {
        debug!("Updating record for user: {:?}", user);

        let mut client = self.client().unwrap();

        let new_version = Uuid::new_v4();
        let new_updated = Utc::now();

        let rows = client.query(
            "UPDATE users SET username = $1, email = $2, display_name = $3, password = $4,
                    version = $5, updated = $6
                    WHERE user_id = $7
                    RETURNING *",
            &[
                &user.data.username,
                &user.data.email,
                &user.data.display_name,
                &user.data.password,
                &new_version,
                &new_updated,
                &user.identity.id,
            ],
        )?;

        if rows.is_empty() {
            Err(PersistUserError::UserNotFound)
        } else {
            let result = rows.get(0).unwrap().into();

            debug!("Updated record for user: {:?}", result);
            Ok(result)
        }
    }
}

impl From<postgres::Error> for PersistUserError {
    fn from(error: postgres::Error) -> Self {
        warn!("Error creating user in database: {:?}", error);

        error
            .source()
            .and_then(|e| e.downcast_ref::<postgres::error::DbError>())
            .map(|e| match e.constraint() {
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
    use std::str::FromStr;
    use test_env_log::test;
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
        let seeded_password = seeded_user.password.clone();
        let seeded_user_entity = UserEntity::from(seeded_user);
        assert_that(&user.identity).is_equal_to(seeded_user_entity.identity);
        assert_that(&user.data.username).is_equal_to(seeded_user_entity.data.username);
        assert_that(&user.data.email).is_equal_to(seeded_user_entity.data.email);
        assert_that(&user.data.display_name).is_equal_to(seeded_user_entity.data.display_name);
        assert_that(&user.data.password.verify(seeded_password)).is_equal_to(true);
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
        let seeded_password = seeded_user.password.clone();
        let seeded_user_entity = UserEntity::from(seeded_user);
        assert_that(&user.identity).is_equal_to(seeded_user_entity.identity);
        assert_that(&user.data.username).is_equal_to(seeded_user_entity.data.username);
        assert_that(&user.data.email).is_equal_to(seeded_user_entity.data.email);
        assert_that(&user.data.display_name).is_equal_to(seeded_user_entity.data.display_name);
        assert_that(&user.data.password.verify(seeded_password)).is_equal_to(true);
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
        let seeded_password = seeded_user.password.clone();
        let seeded_user_entity = UserEntity::from(seeded_user);
        assert_that(&user.identity).is_equal_to(seeded_user_entity.identity);
        assert_that(&user.data.username).is_equal_to(seeded_user_entity.data.username);
        assert_that(&user.data.email).is_equal_to(seeded_user_entity.data.email);
        assert_that(&user.data.display_name).is_equal_to(seeded_user_entity.data.display_name);
        assert_that(&user.data.password.verify(seeded_password)).is_equal_to(true);
    }

    #[test]
    fn test_create_user() {
        let database = TestDatabaseWrapper::new();
        let user = UserData {
            username: "testuser".parse().unwrap(),
            email: "testuser@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_plaintext("Pa55word").unwrap(),
        };

        let created_user = database.wrapper.create_user(user.clone()).unwrap();

        assert_that(&created_user.data).is_equal_to(&user);

        let loaded_user = database
            .wrapper
            .get_user_by_id(&created_user.identity.id)
            .unwrap();
        assert_that(&loaded_user).is_equal_to(created_user);
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

        let user = UserData {
            username: "testuser".parse().unwrap(),
            email: "new@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_plaintext("Pa55word").unwrap(),
        };

        let created_user = database.wrapper.create_user(user.clone());

        assert_that(&created_user)
            .is_err()
            .is_equal_to(PersistUserError::DuplicateUsername);
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

        let user = UserData {
            username: "TestUser".parse().unwrap(),
            email: "new@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_plaintext("Pa55word").unwrap(),
        };

        let created_user = database.wrapper.create_user(user.clone());

        assert_that(&created_user)
            .is_err()
            .is_equal_to(PersistUserError::DuplicateUsername);
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

        let user = UserData {
            username: "new_username".parse().unwrap(),
            email: "testuser@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_plaintext("Pa55word").unwrap(),
        };

        let created_user = database.wrapper.create_user(user.clone());

        assert_that(&created_user)
            .is_err()
            .is_equal_to(PersistUserError::DuplicateEmail);
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

        let user = UserData {
            username: "new_username".parse().unwrap(),
            email: "TestUser@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_plaintext("Pa55word").unwrap(),
        };

        let created_user = database.wrapper.create_user(user.clone());

        assert_that(&created_user)
            .is_err()
            .is_equal_to(PersistUserError::DuplicateEmail);
    }

    #[test]
    fn test_update_user_success() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let mut user = database
            .wrapper
            .get_user_by_username(&Username::from_str("testuser").unwrap())
            .unwrap();

        user.data.username = "newuser".parse().unwrap();
        user.data.email = "newuser@example.com".parse().unwrap();
        user.data.display_name = "New User".parse().unwrap();

        let saved = database.wrapper.update_user(user.clone());

        assert_that(&saved).is_ok();

        let saved = saved.unwrap();
        assert_that(&saved.identity.id).is_equal_to(&user.identity.id);
        assert_that(&saved.identity.created).is_equal_to(&user.identity.created);
        assert_that(&saved.identity.version).is_not_equal_to(&user.identity.version);
        assert_that(&saved.identity.updated).is_not_equal_to(&user.identity.updated);
        assert_that(&saved.data).is_equal_to(&user.data);
    }

    #[test]
    fn test_update_user_reload() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let mut user = database
            .wrapper
            .get_user_by_username(&Username::from_str("testuser").unwrap())
            .unwrap();

        user.data.username = "newuser".parse().unwrap();
        user.data.email = "newuser@example.com".parse().unwrap();
        user.data.display_name = "New User".parse().unwrap();

        let saved = database.wrapper.update_user(user.clone());

        assert_that(&saved).is_ok();

        let loaded = database.wrapper.get_user_by_id(&user.identity.id).unwrap();
        assert_that(&loaded).is_equal_to(saved.unwrap());
    }

    #[test]
    fn test_update_user_same_values() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            ..Default::default()
        };
        seed(&database, vec![&existing_user]);

        let user = database
            .wrapper
            .get_user_by_username(&Username::from_str("testuser").unwrap())
            .unwrap();

        let saved = database.wrapper.update_user(user.clone());

        assert_that(&saved).is_ok();

        let saved = saved.unwrap();
        assert_that(&saved.identity.id).is_equal_to(&user.identity.id);
        assert_that(&saved.identity.created).is_equal_to(&user.identity.created);
        assert_that(&saved.identity.version).is_not_equal_to(&user.identity.version);
        assert_that(&saved.identity.updated).is_not_equal_to(&user.identity.updated);
        assert_that(&saved.data).is_equal_to(&user.data);
    }

    #[test]
    fn test_update_user_duplicate_email() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            ..Default::default()
        };
        let other_user: User = User {
            username: "testuser2".to_owned(),
            email: "newuser@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            ..Default::default()
        };

        seed(&database, vec![&existing_user, &other_user]);

        let mut user = database
            .wrapper
            .get_user_by_username(&Username::from_str("testuser").unwrap())
            .unwrap();

        user.data.username = "newuser".parse().unwrap();
        user.data.email = "newuser@example.com".parse().unwrap();
        user.data.display_name = "New User".parse().unwrap();

        let saved = database.wrapper.update_user(user.clone());

        assert_that(&saved)
            .is_err()
            .is_equal_to(PersistUserError::DuplicateEmail);
    }

    #[test]
    fn test_update_user_duplicate_username() {
        let database = TestDatabaseWrapper::new();
        let existing_user: User = User {
            username: "testuser".to_owned(),
            email: "testuser@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            ..Default::default()
        };
        let other_user: User = User {
            username: "newuser".to_owned(),
            email: "testuser2@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            ..Default::default()
        };

        seed(&database, vec![&existing_user, &other_user]);

        let mut user = database
            .wrapper
            .get_user_by_username(&Username::from_str("testuser").unwrap())
            .unwrap();

        user.data.username = "newuser".parse().unwrap();
        user.data.email = "newuser@example.com".parse().unwrap();
        user.data.display_name = "New User".parse().unwrap();

        let saved = database.wrapper.update_user(user.clone());

        assert_that(&saved)
            .is_err()
            .is_equal_to(PersistUserError::DuplicateUsername);
    }

    #[test]
    fn test_update_user_unknown_user() {
        let database = TestDatabaseWrapper::new();

        let user = UserEntity {
            identity: Default::default(),
            data: UserData {
                username: "testuser".parse().unwrap(),
                email: "test@example.com".parse().unwrap(),
                display_name: "Test User".parse().unwrap(),
                password: Password::from_hash("abc"),
            },
        };

        let saved = database.wrapper.update_user(user.clone());

        assert_that(&saved)
            .is_err()
            .is_equal_to(PersistUserError::UserNotFound);
    }
}
