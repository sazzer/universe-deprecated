use serde::Serialize;
use universe_users::{DisplayName, EmailAddress, UserEntity, UserID, Username};

/// Representation of a User to return over the API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserID,
    pub username: Username,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailAddress>,
    pub display_name: DisplayName,
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
