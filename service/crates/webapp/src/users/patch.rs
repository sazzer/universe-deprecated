use super::model::User;
use crate::problem::Problem;
use crate::request_id::RequestId;
use rocket::{patch, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use tracing::debug;
use universe_users::*;

#[patch("/users/<user_id>", data = "<patch_data>")]
#[tracing::instrument(skip(user_service, access_token_factory, access_token_encoder))]
pub fn update_user(
    _request_id: RequestId,
    user_id: String,
    patch_data: Json<PatchData>,
    user_service: State<Box<dyn UserService>>,
) -> Result<Json<User>, Problem> {
    debug!("Patch Data: {:?}", patch_data);

    todo!()
}

/// Struct representing the input data for updating a user
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchData<'a> {
    pub display_name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
}
