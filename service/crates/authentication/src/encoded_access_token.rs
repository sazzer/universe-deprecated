use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representation of an encode Access Token.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EncodedAccessToken(pub(crate) String);

impl EncodedAccessToken {
  /// Create a new EncodedAccessToken from a bare string
  pub fn new<S>(s: S) -> Self
  where
    S: Into<String>,
  {
    EncodedAccessToken(s.into())
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;
  use spectral::prelude::*;
  use test_env_log::test;

  #[test]
  fn test_serialize_valid_id() {
    let value = EncodedAccessToken::new("f2c55656-d7a1-4e41-a311-fe653b9b15de");

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
      .is_equal_to(EncodedAccessToken::new(
        "f2c55656-d7a1-4e41-a311-fe653b9b15de",
      ));
  }
}
