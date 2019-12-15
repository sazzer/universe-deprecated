use super::{service::UserRepository, Password, UserData, UserEntity, UserID, Username};
use crate::database::Database;
use crate::entity::Identity;
use log::debug;
use r2d2_postgres::PostgresConnectionManager;
use std::sync::Arc;

/// User Repository that works in terms of a PostgreSQL database connection
pub struct PostgresUserRepository {
    database: Arc<dyn Database<PostgresConnectionManager>>,
}

impl PostgresUserRepository {
    pub fn new(database: Arc<dyn Database<PostgresConnectionManager>>) -> Self {
        PostgresUserRepository { database: database }
    }
}
impl UserRepository for PostgresUserRepository {
    fn get_by_username(&self, username: Username) -> Option<UserEntity> {
        debug!("Looking up username {:?}", username);

        let client = self.database.client().ok()?;
        let result: postgres::rows::Rows = client
            .query(
                &"SELECT * FROM users WHERE username = $1".to_owned(),
                &[username.as_ref()],
            )
            .ok()?;

        if result.is_empty() {
            debug!("No user found with username {:?}", username);
            None
        } else {
            let user_row = result.get(0);
            let user = UserEntity {
                identity: Identity {
                    id: UserID(user_row.get("user_id")),
                    version: user_row.get("version"),
                    created: user_row.get("created"),
                    updated: user_row.get("updated"),
                },
                data: UserData {
                    username: Username(user_row.get("username")),
                    email: user_row.get("email"),
                    display_name: user_row.get("display_name"),
                    password: Password::from_hash(user_row.get::<&str, String>("password")),
                },
            };
            debug!("User found with username {:?}: {:?}", username, user);
            Some(user)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use crate::users::*;
    use chrono::Utc;
    use speculate::speculate;
    use uuid::Uuid;

    speculate! {
        before {
            let _ = env_logger::try_init();
            let database = TestDatabase::new();
            let test_subject = PostgresUserRepository::new(database.database.clone());
        }

        describe "get_by_username" {
            it "Returns None for an unknown user" {
                let result = test_subject.get_by_username(Username("unknown".to_owned()));
                assert_eq!(None, result);
            }

            it "Returns correctly for a known user" {
                let id = Uuid::new_v4();
                let version = Uuid::new_v4();
                let now = Utc::now();
                database.database.client().unwrap().query("INSERT INTO USERS(
                    user_id, version, created, updated, username, email, display_name, password)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", &[
                        &id,
                        &version,
                        &now,
                        &now,
                        &"knownuser",
                        &"test@example.com",
                        &"Test User",
                        &"abc"
                    ]).unwrap();

                let expected = UserEntity {
                            identity: crate::entity::Identity {
                                id: UserID(id),
                                version: version,
                                created: now,
                                updated: now,
                            },
                            data: UserData {
                                username: Username("knownuser".to_owned()),
                                display_name: "Test User".to_owned(),
                                email: "test@example.com".to_owned(),
                                password: Password::from_hash("abc"),
                            },
                        };

                let result = test_subject.get_by_username(Username("knownuser".to_owned()));
                assert_eq!(Some(expected), result);
            }
        }
    }
}
