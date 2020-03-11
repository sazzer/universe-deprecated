use crate::{build_headers, build_json_body, build_rewrite_headers, regex_replace, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot, dynamic_redaction};
use rocket::http::ContentType;
use serde_json::{json, Value};
use spectral::prelude::*;
use test_env_log::test;
use universe_testdata::{seed, User};

fn test_failed_login(service: ServiceWrapper, body: Value) {
  let req = service
    .post("/login")
    .header(ContentType::JSON)
    .body(body.to_string());
  let mut response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
  HTTP/1.1 400 .
  Content-Type: application/problem+json
  Server: Rocket
  "###);
  assert_json_snapshot!(build_json_body(&mut response), @r###"
  {
    "status": 400,
    "title": "Invalid Username or Password",
    "type": "tag:universe,2020:users/problems/login_failure"
  }
  "###);
}

#[test]
fn test_post_empty_object() {
  let service = ServiceWrapper::default();

  test_failed_login(service, json!({}));
}

#[test]
fn test_unknown_user() {
  let service = ServiceWrapper::default();

  test_failed_login(
    service,
    json!({
      "username": "unknown",
      "password": "Pa55word",
    }),
  );
}

#[test]
fn test_invalid_password() {
  let service = ServiceWrapper::default();

  let user = User {
    username: "testuser".to_owned(),
    password: "Pa55word".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  test_failed_login(
    service,
    json!({
      "username": "testuser",
      "password": "incorrect",
    }),
  );
}

#[test]
fn test_successful_login() {
  let service = ServiceWrapper::default();
  let user = User {
    username: "testuser".to_owned(),
    password: "Pa55word".to_owned(),
    email: "testing@example.com".to_owned(),
    display_name: "Test User".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  let req = service.post("/login").header(ContentType::JSON).body(
    json!({
        "username": "testuser",
        "password": "Pa55word"
    })
    .to_string(),
  );
  let mut response = req.dispatch();

  assert_snapshot!(build_rewrite_headers(&response, |h| {
    regex_replace(h, r#"/users/[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}"#, "/users/d4ebcc15-ddf2-45e4-b263-892984b0e248")
  }), @r###"
  HTTP/1.1 200 OK.
  Content-Type: application/json
  Link: </users/d4ebcc15-ddf2-45e4-b263-892984b0e248>; rel="canonical"
  Server: Rocket
  "###);
  assert_json_snapshot!(build_json_body(&mut response), {
    ".id" => dynamic_redaction(move |value, _| {
      assert_that(&value.as_str()).is_some().is_equal_to(user.user_id.to_string().as_str());
      "[uuid]"
    }),
    ".accessToken.expiry" => "[expiry-date]",
    ".accessToken.token" => "[token-jwt]",
  },
  @r###"
    {
      "accessToken": {
        "expiry": "[expiry-date]",
        "token": "[token-jwt]"
      },
      "displayName": "Test User",
      "email": "testing@example.com",
      "id": "[uuid]",
      "username": "testuser"
    }
    "###);
}
