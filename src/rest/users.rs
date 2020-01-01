use super::problem::Problem;
use crate::users::{UserEntity, UserID, UserIDParseError, UserService, Username};
use log::debug;
use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

/// Shape of a User as returned by the REST API
#[derive(Serialize)]
pub struct UserModel {
    #[serde(rename = "self")]
    self_url: String,

    id: UserID,
    version: Uuid,

    username: Username,
    display_name: String,
    email: String,
}

impl From<UserIDParseError> for Problem {
    /// Handle converting an error from parsing a User ID as if the user itself couldn't be found.
    /// This then means we treat an invalid User ID as if it was a valid one for a user that doesn't exist.
    fn from(_: UserIDParseError) -> Self {
        debug!("Failed to parse an invalid User ID, so acting as if the user doesn't exist");
        user_not_found()
    }
}

impl From<UserEntity> for UserModel {
    /// Handle converting a User Entity into a User Model to return over the API
    fn from(user: UserEntity) -> Self {
        UserModel {
            self_url: format!("/api/users/{}", user.identity.id),

            id: user.identity.id,
            version: user.identity.version,

            username: user.data.username,
            display_name: user.data.display_name,
            email: user.data.email,
        }
    }
}

/// Actual handler to get a User by their unique ID
#[get("/users/<id>")]
pub fn get_user_by_id(
    id: String,
    user_service: State<Arc<dyn UserService>>,
) -> Result<Json<UserModel>, Problem> {
    let user_id: UserID = id.parse()?;

    let user = user_service.get_user_by_id(user_id).ok_or_else(|| {
        debug!("Failed to find user with ID {}", user_id.clone());
        user_not_found()
    })?;

    Ok(Json(user.into()))
}

fn user_not_found() -> Problem {
    Problem {
        r#type: "tag:universe,2019:problems/users/not-found".to_owned(),
        title: "User not found".to_owned(),
        status: 404,
        ..Default::default()
    }
}
