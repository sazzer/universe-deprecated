use super::{DisplayName, EmailAddress, Password, UserID, Username};
use universe_entity::Identity;

/// Struct to represent the data about a single user record
#[derive(Debug, PartialEq, Clone)]
pub struct UserData {
    pub username: Username,
    pub email: EmailAddress,
    pub display_name: DisplayName,
    pub password: Password,
}

/// Type to represent the entity that is a persisted user record
#[derive(Debug, PartialEq, Clone)]
pub struct UserEntity {
    pub identity: Identity<UserID>,
    pub data: UserData,
}

#[cfg(test)]
impl From<universe_testdata::User> for UserEntity {
    fn from(user: universe_testdata::User) -> UserEntity {
        UserEntity {
            identity: Identity {
                id: UserID::from_uuid(user.user_id),
                version: user.version,
                created: user.created,
                updated: user.updated,
            },
            data: UserData {
                username: user.username.parse().unwrap(),
                email: user.email.parse().unwrap(),
                display_name: user.display_name.parse().unwrap(),
                password: Password::from_hash(user.password),
            },
        }
    }
}
