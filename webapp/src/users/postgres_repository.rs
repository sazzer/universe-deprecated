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
            .query("SELECT * FROM users WHERE username = $1", &[&username.0])
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
