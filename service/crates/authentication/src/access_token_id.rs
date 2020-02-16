use serde::Serialize;
use uuid::Uuid;

/// Representation of an AccessToken ID of some AccessToken.
///
/// An AccessToken ID is any valid String.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct AccessTokenID(String);

impl AccessTokenID {
  /// Create a new AccessTokenID from a bare string
  pub fn new<S>(s: S) -> Self
  where
    S: Into<String>,
  {
    AccessTokenID(s.into())
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;
  use spectral::prelude::*;
  use test_env_log::test;

  #[test]
  fn test_serialize_valid_id() {
    let value = AccessTokenID("f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap());

    let serialized = serde_json::to_value(value);
    assert_that(&serialized)
      .is_ok()
      .is_equal_to(json!("f2c55656-d7a1-4e41-a311-fe653b9b15de"));
  }
}
