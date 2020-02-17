use super::AccessTokenID;
use chrono::{DateTime, Utc};
use universe_users::UserID;

/// Representation of an Access Token used to access APIs
#[derive(Debug, PartialEq)]
pub struct AccessToken {
  pub access_token_id: AccessTokenID,
  pub user_id: UserID,
  pub created: DateTime<Utc>,
  pub expires: DateTime<Utc>,
}
