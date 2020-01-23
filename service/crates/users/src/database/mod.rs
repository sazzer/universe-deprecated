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
}
