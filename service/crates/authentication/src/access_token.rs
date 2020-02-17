use super::AccessTokenID;
use chrono::{DateTime, Duration, Utc};
use universe_users::UserID;

/// Representation of an Access Token used to access APIs
#[derive(Debug, PartialEq)]
pub struct AccessToken {
  pub access_token_id: AccessTokenID,
  pub user_id: UserID,
  pub created: DateTime<Utc>,
  pub expires: DateTime<Utc>,
}

/// Factory that can be used to generate Access Tokens for Users
pub struct AccessTokenFactory {
  expiry: Duration,
}

impl AccessTokenFactory {
  /// Construct a new Access Token Factory
  ///
  /// # Arguments
  /// * `expiry` The duration that an access token must last
  ///
  /// # Returns
  /// The Access Token Factory
  pub fn new(expiry: Duration) -> Self {
    AccessTokenFactory { expiry }
  }

  /// Build a new Access Token for the given User ID
  ///
  /// # Arguments
  /// * `user_id` The ID of the User to create an access token for
  ///
  /// # Returns
  /// The Access Token
  pub fn build(&self, user_id: &UserID) -> AccessToken {
    let now = Utc::now();

    AccessToken {
      access_token_id: Default::default(),
      user_id: user_id.clone(),
      created: now,
      expires: now + self.expiry,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use spectral::prelude::*;

  #[test]
  fn access_token_factory() {
    let factory = AccessTokenFactory::new(Duration::days(1));
    let user_id: UserID = Default::default();

    let access_token = factory.build(&user_id);
    assert_that(&access_token.user_id).is_equal_to(user_id);
    assert_that(&access_token.expires).is_equal_to(access_token.created + Duration::days(1));
  }
}
