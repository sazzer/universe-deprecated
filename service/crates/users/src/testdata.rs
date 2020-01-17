use crate::{Password, UserData, UserEntity, UserID};
use universe_entity::Identity;
use universe_testdata::User;

impl From<User> for UserEntity {
    /// Convert the test User structure into the User Entity
    fn from(user: User) -> Self {
        UserEntity {
            identity: Identity {
                id: UserID::from_uuid(user.user_id),
                version: user.version,
                created: user.created,
                updated: user.updated,
            },
            data: UserData {
                username: user.username.parse().unwrap(),
                email: user.email,
                display_name: user.display_name,
                password: Password::from_hash(user.password),
            },
        }
    }
}
