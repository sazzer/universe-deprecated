use serde::Serialize;
use universe_users::{UserEntity, UserID, Username};

/// Representation of a User to return over the API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserID,
    pub username: Username,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub display_name: String,
}

impl From<UserEntity> for User {
    fn from(user: UserEntity) -> Self {
        Self {
            id: user.identity.id,
            username: user.data.username,
            email: Some(user.data.email),
            display_name: user.data.display_name,
        }
    }
}
