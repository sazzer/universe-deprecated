use crate::authentication::ApiAccessToken;
use crate::request_id::RequestId;
use rocket::get;
use rocket_contrib::json::Json;
use universe_users::UserID;

#[get("/access_token/debug/required")]
#[tracing::instrument(skip())]
pub fn get_access_token_required(
  _request_id: RequestId,
  access_token: ApiAccessToken,
) -> Json<UserID> {
  Json(access_token.access_token.user_id)
}

#[get("/access_token/debug/optional")]
#[tracing::instrument(skip())]
pub fn get_access_token_optional(
  _request_id: RequestId,
  access_token: Option<ApiAccessToken>,
) -> Json<&'static str> {
  match access_token {
    Some(_) => Json("Access Token Present"),
    None => Json("Access Token Absent"),
  }
}
