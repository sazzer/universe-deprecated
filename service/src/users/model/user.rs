use super::{Password, UserID, Username};
use crate::entity::{Entity, Identity};

/// Struct to represent the data about a single user record
#[derive(Debug, PartialEq, Clone)]
pub struct UserData {
    pub username: Username,
    pub email: String,
    pub display_name: String,
    pub password: Password,
}

/// Type to represnt the entity that is a persisted user record
pub type UserEntity = Entity<UserID, UserData>;

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
