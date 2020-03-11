use super::model::{AccessToken, AuthenticatedUser};
use crate::problem::Problem;
use crate::request_id::RequestId;
use rocket::{post, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use tracing::debug;
use universe_authentication::{encoder::AccessTokenEncoder, AccessTokenFactory};
use universe_users::*;

#[post("/login", data = "<authentication>")]
#[tracing::instrument(skip(user_service, access_token_factory, access_token_encoder))]
pub fn authenticate_user(
  _request_id: RequestId,
  authentication: Json<Authentication>,
  user_service: State<Box<dyn UserService>>,
  access_token_factory: State<AccessTokenFactory>,
  access_token_encoder: State<AccessTokenEncoder>,
) -> Result<AuthenticatedUser, Problem> {
  debug!("Authentication: {:?}", authentication);

  let username: Result<Username, UsernameParseError> =
    authentication.username.unwrap_or("").parse();

  let user = username
    .ok()
    .and_then(|username| user_service.get_user_by_username(&username));

  if let Some(user) = user {
    let valid_password = user
      .data
      .password
      .verify(authentication.password.unwrap_or(""));

    if valid_password {
      let access_token = access_token_factory.build(&user.identity.id);
      debug!(
        "Access Token for user {:?}: {:?}",
        user.identity.id, access_token
      );
      Ok(AuthenticatedUser {
        user: user.into(),
        access_token: AccessToken {
          token: access_token_encoder.encode(&access_token).unwrap(),
          expiry: access_token.expires,
        },
      })
    } else {
      Err(invalid_login_problem())
    }
  } else {
    Err(invalid_login_problem())
  }
}

/// Struct representing the input data for authenticating an existing user
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Authentication<'a> {
  pub username: Option<&'a str>,
  pub password: Option<&'a str>,
}

/// Helper to build a Problem response for a failed login
fn invalid_login_problem() -> Problem {
  Problem {
    r#type: "tag:universe,2020:users/problems/login_failure".to_owned(),
    title: "Invalid Username or Password".to_owned(),
    status: 400,
    ..Default::default()
  }
}
