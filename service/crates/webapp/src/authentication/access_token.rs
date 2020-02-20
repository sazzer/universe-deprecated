use crate::problem::Problem;
use rocket::{
  http::Status,
  request::{FromRequest, Outcome, Request},
  State,
};
use tracing::{debug, warn};
use universe_authentication::{encoder::AccessTokenEncoder, AccessToken, EncodedAccessToken};

#[derive(Debug, PartialEq)]
pub struct ApiAccessToken {
  pub access_token: AccessToken,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiAccessToken {
  type Error = Problem;

  fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    let access_token_encoder: State<AccessTokenEncoder> = request.guard().unwrap();

    match request.headers().get_one("Authorization") {
      None => {
        debug!("No Authorization header present");
        build_failure_response()
      }
      Some(header) => {
        if header.starts_with("Bearer ") {
          let token = &header[7..];
          match access_token_encoder.decode(EncodedAccessToken::new(token)) {
            Ok(access_token) => {
              debug!("Decoded access token: {:?}", access_token);
              Outcome::Success(ApiAccessToken { access_token })
            }
            Err(e) => {
              warn!("Failed to decode access token '{}': {}", token, e);
              build_failure_response()
            }
          }
        } else {
          debug!("Authorization header is not a bearer token");
          build_failure_response()
        }
      }
    }
  }
}

fn build_failure_response() -> Outcome<ApiAccessToken, Problem> {
  Outcome::Failure((
    Status::Forbidden,
    Problem {
      r#type: "tag:universe,2020:problems/authentication/missing-access-token".to_owned(),
      title: "No Access Token was provided on the request".to_owned(),
      status: 403,
      ..Default::default()
    },
  ))
}
