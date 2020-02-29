use super::ApiAccessToken;
use crate::problem::Problem;
use rocket::request::{FromRequest, Outcome, Request};
use universe_authentication::AccessToken;
use universe_users::UserID;

/// Authenticator to determine whether a request is allowed to proceed or not
pub struct Authorizer {
  /// The Access Token to authenticate
  access_token: AccessToken,
}

/// Enumeration of the possible authorization states
#[derive(Debug, PartialEq)]
pub enum Authorized {
  /// Authorization was a success
  Success,
  /// Authorization was a failure
  Failure,
}

impl<'a, 'r> FromRequest<'a, 'r> for Authorizer {
  type Error = Problem;

  fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    let access_token = ApiAccessToken::from_request(request);

    access_token
      .map(|access_token| access_token.access_token)
      .map(|access_token| Authorizer::from_access_token(access_token))
  }
}

impl Authorizer {
  pub fn from_access_token(access_token: AccessToken) -> Self {
    Authorizer { access_token }
  }

  pub fn same_user(&self, user_id: &UserID) -> Authorized {
    if user_id == &self.access_token.user_id {
      Authorized::Success
    } else {
      Authorized::Failure
    }
  }
}

impl Authorized {
  /// Build a Result representing whether the authorization was a success or a failure.
  ///
  /// In the success case this is just a void response.
  /// In the failure case this is a `Problem` that can be returned to the client
  pub fn to_result(self) -> Result<(), Problem> {
    match self {
      Authorized::Success => Ok(()),
      Authorized::Failure => Err(Problem {
        r#type: "tag:universe,2020:problems/authentication/forbidden".to_owned(),
        title: "You are not permitted to perform this request".to_owned(),
        status: 403,
        ..Default::default()
      }),
    }
  }

  /// Combine two authorization results to return success only if both were successful
  pub fn and(self, other: Authorized) -> Authorized {
    match (self, other) {
      (Authorized::Success, Authorized::Success) => Authorized::Success,
      _ => Authorized::Failure,
    }
  }

  /// Combine two authorization results to return success if either were successful
  pub fn or(self, other: Authorized) -> Authorized {
    match (self, other) {
      (Authorized::Failure, Authorized::Failure) => Authorized::Failure,
      _ => Authorized::Success,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Utc;
  use spectral::prelude::*;

  #[test]
  fn test_same_user() {
    let user_id = UserID::default();
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: user_id.clone(),
      created: Utc::now(),
      expires: Utc::now(),
    };

    let authorizer = Authorizer::from_access_token(access_token);
    assert_that(&authorizer.same_user(&user_id)).is_equal_to(Authorized::Success)
  }

  #[test]
  fn test_different_user() {
    let user_id = UserID::default();
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: UserID::default(),
      created: Utc::now(),
      expires: Utc::now(),
    };

    let authorizer = Authorizer::from_access_token(access_token);
    assert_that(&authorizer.same_user(&user_id)).is_equal_to(Authorized::Failure)
  }

  #[test]
  fn test_success_result() {
    let result = Authorized::Success.to_result();
    assert_that(&result).is_ok().is_equal_to(());
  }

  #[test]
  fn test_failure_result() {
    let result = Authorized::Failure.to_result();
    assert_that(&result).is_err().is_equal_to(Problem {
      r#type: "tag:universe,2020:problems/authentication/forbidden".to_owned(),
      title: "You are not permitted to perform this request".to_owned(),
      status: 403,
      ..Default::default()
    });
  }

  #[test]
  fn test_and() {
    assert_that(&Authorized::Success.and(Authorized::Success)).is_equal_to(Authorized::Success);
    assert_that(&Authorized::Success.and(Authorized::Failure)).is_equal_to(Authorized::Failure);
    assert_that(&Authorized::Failure.and(Authorized::Success)).is_equal_to(Authorized::Failure);
    assert_that(&Authorized::Failure.and(Authorized::Failure)).is_equal_to(Authorized::Failure);
  }

  #[test]
  fn test_or() {
    assert_that(&Authorized::Success.or(Authorized::Success)).is_equal_to(Authorized::Success);
    assert_that(&Authorized::Success.or(Authorized::Failure)).is_equal_to(Authorized::Success);
    assert_that(&Authorized::Failure.or(Authorized::Success)).is_equal_to(Authorized::Success);
    assert_that(&Authorized::Failure.or(Authorized::Failure)).is_equal_to(Authorized::Failure);
  }
}
