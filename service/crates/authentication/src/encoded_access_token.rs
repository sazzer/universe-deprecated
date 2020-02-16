use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// Representation of an encode Access Token.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EncodedAccessToken(String);

impl std::fmt::Display for EncodedAccessToken {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Default for EncodedAccessToken {
  fn default() -> Self {
    EncodedAccessToken(Uuid::new_v4().to_string())
  }
}
/// Implementation of the standard `FromStr` trait to allow us to parse any String into an `EncodedAccessToken` object
impl FromStr for EncodedAccessToken {
  type Err = ();

  /// Attempt to parse a string into an EncodedAccessToken object.
  ///
  /// # Arguments
  /// * `s` The string to parse
  ///
  /// # Returns
  /// The result of parsing the encoded AccessToken. Always an `EncodedAccessToken` object
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(EncodedAccessToken(s.to_owned()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;
  use spectral::prelude::*;
  use test_env_log::test;

  #[test]
  fn test_parse_valid_id() {
    let value: Result<EncodedAccessToken, ()> = "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse();

    assert_that(&value).is_ok().is_equal_to(EncodedAccessToken(
      "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
    ));
  }

  #[test]
  fn test_serialize_valid_id() {
    let value = EncodedAccessToken("f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap());

    let serialized = serde_json::to_value(value);
    assert_that(&serialized)
      .is_ok()
      .is_equal_to(json!("f2c55656-d7a1-4e41-a311-fe653b9b15de"));
  }

  #[test]
  fn test_deserialize_valid_id() {
    let value = json!("f2c55656-d7a1-4e41-a311-fe653b9b15de");

    let deserialized: Result<EncodedAccessToken, _> = serde_json::from_value(value);
    assert_that(&deserialized)
      .is_ok()
      .is_equal_to(EncodedAccessToken(
        "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
      ));
  }
}
