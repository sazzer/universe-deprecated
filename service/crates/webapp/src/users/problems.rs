use crate::problem::{missing_error, unexpected_error, validation_error, Problem, ValidationError};
use universe_users::*;

/// Helper to build a Problem response for an unknown user
pub fn unknown_user_problem() -> Problem {
  Problem {
    r#type: "tag:universe,2020:users/problems/unknown-user".to_owned(),
    title: "The requested user could not be found".to_owned(),
    status: 404,
    ..Default::default()
  }
}

impl From<Vec<ValidationError>> for Problem {
  fn from(e: Vec<ValidationError>) -> Problem {
    validation_error(e)
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

impl From<UsernameParseError> for ValidationError {
  fn from(e: UsernameParseError) -> Self {
    match e {
      UsernameParseError::Blank => missing_error("username"),
    }
  }
}

impl From<DisplayNameParseError> for ValidationError {
  fn from(e: DisplayNameParseError) -> Self {
    match e {
      DisplayNameParseError::Blank => missing_error("displayName"),
    }
  }
}

impl From<EmailAddressParseError> for ValidationError {
  fn from(e: EmailAddressParseError) -> Self {
    match e {
      EmailAddressParseError::Blank => missing_error("email"),
      EmailAddressParseError::Malformed => ValidationError {
        r#type: "tag:universe,2020:users/validation-errors/email/malformed".to_owned(),
        title: "Email Address was malformed".to_owned(),
        field: "email".to_owned(),
      },
    }
  }
}

impl From<PasswordHashError> for ValidationError {
  fn from(e: PasswordHashError) -> Self {
    match e {
      PasswordHashError::Blank => missing_error("password"),
      PasswordHashError::HashError(_) => ValidationError {
        r#type: "tag:universe,2020:validation-errors/password/invalid-password".to_owned(),
        title: "The password was invalid".to_owned(),
        field: "password".to_owned(),
      },
    }
  }
}

impl From<RegisterUserError> for Problem {
  fn from(e: RegisterUserError) -> Self {
    match e {
      RegisterUserError::ValidationError(errors) => {
        validation_error(errors.iter().map(|e| e.into()).collect())
      }
      _ => unexpected_error(),
    }
  }
}

#[derive(Debug)]
pub struct ValidationErrors {
  pub errors: Vec<ValidationError>,
}

impl std::fmt::Display for ValidationErrors {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?}", self.errors)
  }
}

impl std::error::Error for ValidationErrors {}

impl From<UpdateUserError> for Problem {
  fn from(e: UpdateUserError) -> Self {
    match e {
      UpdateUserError::ValidationError(errors) => {
        validation_error(errors.iter().map(|e| e.into()).collect())
      }
      UpdateUserError::UnknownUser => unknown_user_problem(),
      UpdateUserError::UpdateError(e) if e.is::<ValidationErrors>() => e
        .downcast_ref::<ValidationErrors>()
        .unwrap()
        .errors
        .clone()
        .into(),
      _ => unexpected_error(),
    }
  }
}
