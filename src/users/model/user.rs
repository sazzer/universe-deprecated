use super::{Password, UserID, Username};
use crate::entity::Entity;

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
