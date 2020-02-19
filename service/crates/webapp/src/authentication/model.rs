use crate::users::model::User;
use chrono::{DateTime, Utc};
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
