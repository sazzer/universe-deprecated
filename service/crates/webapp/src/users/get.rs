use super::{model::User, problems::unknown_user_problem};
use crate::problem::Problem;
use crate::request_id::RequestId;
use rocket::{get, http::Status, Response, State};
use rocket_contrib::json::Json;
use tracing::warn;
use universe_users::{UserID, UserService, Username};

#[get("/users/<user_id>")]
#[tracing::instrument(skip(user_service))]
pub fn get_user_by_id(
    _request_id: RequestId,
    user_id: String,
    user_service: State<Box<dyn UserService>>,
) -> Result<Json<User>, Problem> {
    let user_id: UserID = user_id.parse().map_err(|e| {
        warn!("Invalid User ID: {}", e);
        unknown_user_problem()
    })?;

    let user = user_service
        .get_user_by_id(&user_id)
        .ok_or_else(unknown_user_problem)?;

    Ok(Json(user.into()))
}

#[get("/usernames/<username>")]
#[tracing::instrument(skip(user_service))]
pub fn get_username(
    _request_id: RequestId,
    username: String,
    user_service: State<Box<dyn UserService>>,
) -> Result<Response, Problem> {
    let username: Username = username.parse().map_err(|e| {
        warn!("Invalid Username: {}", e);
        unknown_user_problem()
    })?;

    let user = user_service
        .get_user_by_username(&username)
        .ok_or_else(unknown_user_problem)?;

    let response = Response::build()
        .status(Status::NoContent)
        .raw_header(
            "Link",
            format!("</users/{}>; rel=\"canonical\"", user.identity.id),
        )
        .finalize();

    Ok(response)
}
