use crate::{AccessToken, EncodedAccessToken};
use frank_jwt::{encode, Algorithm};
use serde_json::json;

/// Means by which we can convert an Access Token to and from the Encoded form
pub struct AccessTokenEncoder {
  key: String,
  algorithm: Algorithm,
}

impl AccessTokenEncoder {
  /// Create a new Access Token Encoder to use
  ///
  /// # Arguments
  /// * `key` The signing key to use
  ///
  /// # Returns
  /// The encoder to use
  pub fn new<K>(key: K) -> Self
  where
    K: Into<String>,
  {
    AccessTokenEncoder {
      key: key.into(),
      algorithm: Algorithm::HS512,
    }
  }

  /// Encode the gven Access Token into an EncodedAccessToken for providing to clients
  ///
  /// # Arguments
  /// * `input` The access token to encode
  ///
  /// # Returns
  /// The result of encoding the access token
  pub fn encode(&self, input: &AccessToken) -> EncodedAccessToken {
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "sub": input.user_id,
      "jti": input.access_token_id,
      "exp": input.expires.timestamp(),
      "nbf": input.created.timestamp(),
      "iat": input.created.timestamp(),
    });

    let jwt = encode(json!({}), &self.key, &payload, self.algorithm).unwrap();

    EncodedAccessToken::new(jwt)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Duration;
  use chrono::Utc;
  use frank_jwt::{decode, Algorithm, ValidationOptions};
  use spectral::prelude::*;

  #[test]
  fn test_encode() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now(),
      expires: Utc::now() + Duration::days(1),
    };

    let encoder = AccessTokenEncoder::new("key");
    let encoded = encoder.encode(&access_token);
    let raw_jwt = format!("{}", encoded);

    let (header, payload) = decode(
      raw_jwt.as_str(),
      &"key",
      Algorithm::HS512,
      &ValidationOptions::default(),
    )
    .expect("Decode Access Token");
    assert_that(&header).is_equal_to(json!({
      "alg": "HS512",
      "typ": "JWT"
    }));
    assert_that(&payload).is_equal_to(json!({
      "aud": "universe",
      "iss": "universe",
      "exp": access_token.expires.timestamp(),
      "iat": access_token.created.timestamp(),
      "nbf": access_token.created.timestamp(),
      "sub": access_token.user_id,
      "jti": access_token.access_token_id,
    }));
  }
}
