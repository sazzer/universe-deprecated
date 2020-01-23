use super::model::User;
use crate::problem::Problem;
use rocket::{get, State};
use rocket_contrib::json::Json;
use tracing::warn;
use universe_users::{UserID, UserService};

#[get("/users/<user_id>")]
pub fn get_user_by_id(
    user_id: String,
    user_service: State<Box<dyn UserService>>,
) -> Result<Json<User>, Problem> {
    let user_id: UserID = user_id.parse().map_err(|e| {
        warn!("Invalid User ID: {}", e);
        unknown_user_problem()
    })?;

    let user = user_service
        .get_user_by_id(&user_id)
        .ok_or_else(|| unknown_user_problem())?;

    Ok(Json(user.into()))
}

/// Helper to build a Problem response for an unknown user
fn unknown_user_problem() -> Problem {
    Problem {
        r#type: "tag:universe,2020:users/problems/unknown-user".to_owned(),
        title: "The requested user could not be found".to_owned(),
        status: 404,
        ..Default::default()
    }
}
