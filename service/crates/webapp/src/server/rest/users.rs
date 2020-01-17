use super::problem::Problem;
use crate::server::request_id::RequestId;
use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::sync::Arc;
use tracing::debug;
use universe_users::{UserEntity, UserID, UserService, Username};
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
#[tracing::instrument(skip(user_service))]
#[get("/users/<id>")]
pub fn get_user_by_id(
    id: String,
    user_service: State<Arc<dyn UserService>>,
    _request_id: RequestId,
) -> Result<Json<UserModel>, Problem> {
    let user_id: UserID = id.parse().map_err(|e| {
        debug!(
            "Failed to parse an invalid User ID, so acting as if the user doesn't exist: {}",
            e
        );
        user_not_found()
    })?;

    let user = user_service.get_user_by_id(&user_id).ok_or_else(|| {
        debug!("Failed to find user with ID {}", user_id.clone());
        user_not_found()
    })?;

    Ok(Json(user.into()))
}

/// Build a Problem to represent a failure to find a user
fn user_not_found() -> Problem {
    Problem {
        r#type: "tag:universe,2019:problems/users/not-found".to_owned(),
        title: "User not found".to_owned(),
        status: 404,
        ..Default::default()
    }
}
