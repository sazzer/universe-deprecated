use crate::problem::{Problem, ValidationError};
use universe_users::UserValidationError;

/// Helper to build a Problem response for an unknown user
pub fn unknown_user_problem() -> Problem {
  Problem {
    r#type: "tag:universe,2020:users/problems/unknown-user".to_owned(),
    title: "The requested user could not be found".to_owned(),
    status: 404,
    ..Default::default()
  }
}

impl From<&UserValidationError> for ValidationError {
  fn from(e: &UserValidationError) -> Self {
    match e {
      UserValidationError::DuplicateUsername => ValidationError {
        r#type: "tag:universe,2020:users/validation-errors/username/duplicate".to_owned(),
        title: "The username is already registered".to_owned(),
        field: "username".to_owned(),
      },
      UserValidationError::DuplicateEmail => ValidationError {
        r#type: "tag:universe,2020:users/validation-errors/email/duplicate".to_owned(),
        title: "The email address is already registered".to_owned(),
        field: "email".to_owned(),
      },
    }
  }
}
