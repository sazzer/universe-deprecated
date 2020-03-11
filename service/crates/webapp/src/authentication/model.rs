use crate::{headers::Link, users::model::User};
use chrono::{DateTime, Utc};
use rocket::{
  http::Status,
  response::{Responder, Response},
  Request,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use universe_authentication::EncodedAccessToken;

/// Representation of an Access Token for a User
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
  pub token: EncodedAccessToken,
  pub expiry: DateTime<Utc>,
}

/// Representation of a User + Access Token they are authenticated with
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
  #[serde(flatten)]
  pub user: User,
  pub access_token: AccessToken,
}

impl<'a> Responder<'a> for AuthenticatedUser {
  /// Generate a Rocket response for the User
  fn respond_to(self, req: &Request) -> Result<Response<'a>, Status> {
    Response::build()
      .merge(Json(&self).respond_to(req)?)
      .header(Link::from_href(format!("/users/{}", self.user.id)).with_rel("canonical"))
      .ok()
  }
}
