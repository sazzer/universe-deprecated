use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

/// Representation of a AccessToken ID of some AccessToken in the system.
///
/// An AccessToken ID is any valid String.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct AccessTokenID(String);

impl std::fmt::Display for AccessTokenID {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Default for AccessTokenID {
  fn default() -> Self {
    AccessTokenID(Uuid::new_v4().to_string())
  }
}
/// Implementation of the standard `FromStr` trait to allow us to parse any String into an `AccessTokenID` object
impl FromStr for AccessTokenID {
  type Err = ();

  /// Attempt to parse a string into an AccessTokenID object.
  ///
  /// A AccessToken ID is any valid String.
  ///
  /// # Arguments
  /// * `s` The string to parse
  ///
  /// # Returns
  /// The result of parsing the AccessToken ID. Always an `AccessTokenID` object
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(AccessTokenID(s.to_owned()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;
  use spectral::prelude::*;
  use test_env_log::test;

  #[test]
  fn test_parse_valid_user_id() {
    let user_id: Result<AccessTokenID, ()> = "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse();

    assert_that(&user_id).is_ok().is_equal_to(AccessTokenID(
      "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
    ));
  }

  #[test]
  fn test_serialize_valid_id() {
    let user_id = AccessTokenID("f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap());

    let serialized = serde_json::to_value(user_id);
    assert_that(&serialized)
      .is_ok()
      .is_equal_to(json!("f2c55656-d7a1-4e41-a311-fe653b9b15de"));
  }
}
