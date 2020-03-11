use crate::headers::*;
use chrono::{DateTime, Utc};
use rocket::{
    http::{
        hyper::header::{ETag, EntityTag, HttpDate, LastModified},
        Status,
    },
    response::{Responder, Response},
    Request,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use universe_users::{DisplayName, EmailAddress, UserEntity, UserID, Username};
use uuid::Uuid;

/// Representation of a User to return over the API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserID,
    pub username: Username,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailAddress>,
    pub display_name: DisplayName,
    #[serde(skip_serializing)]
    version: Uuid,
    #[serde(skip_serializing)]
    updated: DateTime<Utc>,
}

impl<'a> Responder<'a> for User {
    /// Generate a Rocket response for the User
    fn respond_to(self, req: &Request) -> Result<Response<'a>, Status> {
        Response::build()
            .merge(Json(&self).respond_to(req)?)
            .header(Link::from_href(format!("/users/{}", self.id)).with_rel("self"))
            .header(AcceptPatch("application/merge-patch+json"))
            .header(ETag(EntityTag::new(false, self.version.to_string())))
            .header(LastModified(HttpDate(time::at_utc(time::Timespec::new(
                self.updated.timestamp(),
                0,
            )))))
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
            version: user.identity.version,
            updated: user.identity.updated,
        }
    }
}
