use crate::{AccessToken, AccessTokenID, EncodedAccessToken};
use chrono::{DateTime, TimeZone, Utc};
use frank_jwt::{decode, encode, Algorithm, ValidationOptions};
use serde_json::json;
use tracing::{debug, warn};
use universe_users::UserID;

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
  pub fn encode(&self, input: &AccessToken) -> Result<EncodedAccessToken, EncodeError> {
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "sub": input.user_id,
      "jti": input.access_token_id,
      "exp": input.expires.timestamp(),
      "nbf": input.created.timestamp(),
      "iat": input.created.timestamp(),
    });

    let jwt = encode(json!({}), &self.key, &payload, self.algorithm)?;

    Ok(EncodedAccessToken::new(jwt))
  }

  pub fn decode(&self, input: EncodedAccessToken) -> Result<AccessToken, DecodeError> {
    debug!("Decoding access token: {:?}", input);
    let (_, payload) = decode(
      input.0.as_str(),
      &self.key,
      Algorithm::HS512,
      &ValidationOptions::default(),
    )?;

    let jti: AccessTokenID = payload
      .get("jti")
      .filter(|v| v.is_string())
      .map(|v| v.as_str().unwrap().to_owned())
      .map(AccessTokenID::new)
      .ok_or(DecodeError::MissingAccessTokenField("jti"))?;
    let sub: UserID = payload
      .get("sub")
      .filter(|v| v.is_string())
      .map(|v| v.as_str().unwrap())
      .ok_or(DecodeError::MissingAccessTokenField("sub"))?
      .parse()?;
    let exp: DateTime<Utc> = payload
      .get("exp")
      .filter(|v| v.is_number())
      .map(|v| v.as_i64().unwrap())
      .map(|v| Utc.timestamp(v, 0))
      .ok_or(DecodeError::MissingAccessTokenField("exp"))?;
    let iat: DateTime<Utc> = payload
      .get("iat")
      .filter(|v| v.is_number())
      .map(|v| v.as_i64().unwrap())
      .map(|v| Utc.timestamp(v, 0))
      .ok_or(DecodeError::MissingAccessTokenField("iat"))?;

    Ok(AccessToken {
      user_id: sub,
      access_token_id: jti,
      expires: exp,
      created: iat,
    })
  }
}

/// Error caused by failing to encode an access token
#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
  #[error("Error encoding Access Token as JWT: {0:?}")]
  EncodeError(#[from] frank_jwt::Error),
}

/// Error caused by failing to decode an access token
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DecodeError {
  #[error("Access token has expired")]
  ExpiredAccessToken,
  #[error("Access token was malformed")]
  MalformedAccessToken,
  #[error("Access token was missing a required field: {0}")]
  MissingAccessTokenField(&'static str),
}

impl From<frank_jwt::Error> for DecodeError {
  fn from(e: frank_jwt::Error) -> Self {
    warn!("Error decoding Access Token as JWT: {:?}", e);
    match e {
      frank_jwt::Error::SignatureExpired => DecodeError::ExpiredAccessToken,
      frank_jwt::Error::ExpirationInvalid => DecodeError::ExpiredAccessToken,
      _ => DecodeError::MalformedAccessToken,
    }
  }
}

impl From<universe_users::UserIDParseError> for DecodeError {
  fn from(e: universe_users::UserIDParseError) -> Self {
    warn!("Malformed User ID decoding Access Token: {:?}", e);
    DecodeError::MissingAccessTokenField("sub")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::{Duration, Timelike};
  use spectral::prelude::*;
  use test_env_log::test;

  #[test]
  fn test_encode() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now(),
      expires: Utc::now() + Duration::days(1),
    };

    let encoder = AccessTokenEncoder::new("key");
    let encoded = encoder.encode(&access_token).expect("Encode Access Token");

    let (header, payload) = decode(
      encoded.0.as_str(),
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

  #[test]
  fn test_encode_decode() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };

    let encoder = AccessTokenEncoder::new("key");
    let encoded = encoder.encode(&access_token).expect("Encode Access Token");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded).is_ok().is_equal_to(access_token);
  }

  #[test]
  fn test_decode_expired() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap() - Duration::days(2),
      expires: Utc::now().with_nanosecond(0).unwrap() - Duration::days(1),
    };

    let encoder = AccessTokenEncoder::new("key");
    let encoded = encoder.encode(&access_token).expect("Encode Access Token");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::ExpiredAccessToken);
  }

  #[test]
  fn test_decode_future() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap() + Duration::days(2),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };

    let encoder = AccessTokenEncoder::new("key");
    let encoded = encoder.encode(&access_token).expect("Encode Access Token");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded).is_ok().is_equal_to(access_token);
  }

  #[test]
  fn test_decode_wrong_format() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "sub": access_token.user_id,
      "jti": access_token.access_token_id,
      "exp": access_token.expires.timestamp(),
      "nbf": access_token.created.timestamp(),
      "iat": access_token.created.timestamp(),
    });

    let jwt = encode(json!({}), &"key", &payload, Algorithm::HS256).unwrap();

    let encoded = EncodedAccessToken::new(jwt);

    let encoder = AccessTokenEncoder::new("key");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::MalformedAccessToken);
  }

  #[test]
  fn test_decode_wrong_key() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "sub": access_token.user_id,
      "jti": access_token.access_token_id,
      "exp": access_token.expires.timestamp(),
      "nbf": access_token.created.timestamp(),
      "iat": access_token.created.timestamp(),
    });

    let jwt = encode(json!({}), &"wrong", &payload, Algorithm::HS512).unwrap();

    let encoded = EncodedAccessToken::new(jwt);

    let encoder = AccessTokenEncoder::new("key");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::MalformedAccessToken);
  }

  #[test]
  fn test_decode_missing_subject() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "jti": access_token.access_token_id,
      "exp": access_token.expires.timestamp(),
      "nbf": access_token.created.timestamp(),
      "iat": access_token.created.timestamp(),
    });

    let jwt = encode(json!({}), &"key", &payload, Algorithm::HS512).unwrap();

    let encoded = EncodedAccessToken::new(jwt);

    let encoder = AccessTokenEncoder::new("key");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::MissingAccessTokenField("sub"));
  }

  #[test]
  fn test_decode_missing_id() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "sub": access_token.user_id,
      "exp": access_token.expires.timestamp(),
      "nbf": access_token.created.timestamp(),
      "iat": access_token.created.timestamp(),
    });

    let jwt = encode(json!({}), &"key", &payload, Algorithm::HS512).unwrap();

    let encoded = EncodedAccessToken::new(jwt);

    let encoder = AccessTokenEncoder::new("key");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::MissingAccessTokenField("jti"));
  }

  #[test]
  fn test_decode_missing_expiry() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "jti": access_token.access_token_id,
      "sub": access_token.user_id,
      "nbf": access_token.created.timestamp(),
      "iat": access_token.created.timestamp(),
    });

    let jwt = encode(json!({}), &"key", &payload, Algorithm::HS512).unwrap();

    let encoded = EncodedAccessToken::new(jwt);

    let encoder = AccessTokenEncoder::new("key");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::ExpiredAccessToken);
  }

  #[test]
  fn test_decode_missing_created() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "jti": access_token.access_token_id,
      "sub": access_token.user_id,
      "exp": access_token.expires.timestamp(),
      "nbf": access_token.created.timestamp(),
    });

    let jwt = encode(json!({}), &"key", &payload, Algorithm::HS512).unwrap();

    let encoded = EncodedAccessToken::new(jwt);

    let encoder = AccessTokenEncoder::new("key");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::MissingAccessTokenField("iat"));
  }

  #[test]
  fn test_decode_malformed_user() {
    let access_token = AccessToken {
      access_token_id: Default::default(),
      user_id: Default::default(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      expires: Utc::now().with_nanosecond(0).unwrap() + Duration::days(1),
    };
    let payload = json!({
      "iss": "universe",
      "aud": "universe",
      "jti": access_token.access_token_id,
      "sub": "userid",
      "exp": access_token.expires.timestamp(),
      "iat": access_token.created.timestamp(),
      "nbf": access_token.created.timestamp(),
    });

    let jwt = encode(json!({}), &"key", &payload, Algorithm::HS512).unwrap();

    let encoded = EncodedAccessToken::new(jwt);

    let encoder = AccessTokenEncoder::new("key");
    let decoded = encoder.decode(encoded);

    assert_that(&decoded)
      .is_err()
      .is_equal_to(DecodeError::MissingAccessTokenField("sub"));
  }
}
