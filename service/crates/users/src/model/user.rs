use super::{Password, UserID, Username};
use universe_entity::Identity;

/// Struct to represent the data about a single user record
#[derive(Debug, PartialEq, Clone)]
pub struct UserData {
    pub username: Username,
    pub email: String,
    pub display_name: String,
    pub password: Password,
}

/// Type to represnt the entity that is a persisted user record
#[derive(Debug, PartialEq, Clone)]
pub struct UserEntity {
    pub identity: Identity<UserID>,
    pub data: UserData,
}
