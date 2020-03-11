use rocket::{
    http::Status,
    response::{Responder, Response},
    Request,
};
use rocket_contrib::json::Json;
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

impl<'a> Responder<'a> for User {
    /// Generate a Rocket response for the User
    fn respond_to(self, req: &Request) -> Result<Response<'a>, Status> {
        Response::build()
            .merge(Json(&self).respond_to(req)?)
            .raw_header("Link", format!("</users/{}>; rel=\"self\"", self.id))
            .ok()
    }
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
