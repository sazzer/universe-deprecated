use crate::model::*;
use crate::service::repository::UserRepository;
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
            .query("SELECT * FROM users WHERE username = $1", &[&username])
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use universe_entity::Identity;
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
        assert_that(&user).is_equal_to(UserEntity {
            identity: Identity {
                id: UserID::from_uuid(seeded_user.user_id),
                version: seeded_user.version,
                created: seeded_user.created,
                updated: seeded_user.updated,
            },
            data: UserData {
                username: seeded_user.username.parse().unwrap(),
                email: seeded_user.email,
                display_name: seeded_user.display_name,
                password: Password::from_hash(seeded_user.password),
            },
        });
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
        assert_that(&user).is_equal_to(UserEntity {
            identity: Identity {
                id: UserID::from_uuid(seeded_user.user_id),
                version: seeded_user.version,
                created: seeded_user.created,
                updated: seeded_user.updated,
            },
            data: UserData {
                username: seeded_user.username.parse().unwrap(),
                email: seeded_user.email,
                display_name: seeded_user.display_name,
                password: Password::from_hash(seeded_user.password),
            },
        });
    }
}
