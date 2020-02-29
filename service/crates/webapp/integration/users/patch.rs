use crate::{build_headers, build_json_body, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot};
use rocket::http::{ContentType, Header, Status};
use serde_json::json;
use spectral::prelude::*;
use test_env_log::test;
use universe_testdata::{seed, User};

fn authenticate_user<'h>(service: &ServiceWrapper, user: &User) -> Header<'h> {
  authenticate(&service, &user.username, &user.password)
}

fn authenticate<'h>(service: &ServiceWrapper, username: &str, password: &str) -> Header<'h> {
  let req = service.post("/login").header(ContentType::JSON).body(
    json!({
        "username": username,
        "password": password
    })
    .to_string(),
  );
  let mut response = req.dispatch();
  assert_that(&response.status()).is_equal_to(Status::Ok);

  let body = build_json_body(&mut response);
  body
    .get("accessToken")
    .and_then(|token| token.get("token"))
    .and_then(|token| token.as_str())
    .map(|token| Header::new("Authorization", format!("Bearer {}", token)))
    .expect("No Access Token returned")
}

#[test]
fn test_patch_unauthorized() {
  let service = ServiceWrapper::default();

  let req = service
    .patch("/users/83C60AD3-2A4F-455B-B685-C16DA785BF6E")
    .header(ContentType::JSON)
    .body(json!({}).to_string());
  let response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
  HTTP/1.1 401 Unauthorized.
  Content-Type: text/html; charset=utf-8
  Server: Rocket
  "###);
}

#[test]
fn test_patch_wrong_user() {
  let service = ServiceWrapper::default();
  let user = User {
    user_id: uuid::Uuid::parse_str("2fcc3850-bb9b-405e-bbab-22978283fef8").unwrap(),
    username: "testuser".to_owned(),
    email: "testing@example.com".to_owned(),
    display_name: "Test User".to_owned(),
    password: "Pa55word".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  let req = service
    .patch("/users/83C60AD3-2A4F-455B-B685-C16DA785BF6E")
    .header(ContentType::JSON)
    .header(authenticate_user(&service, &user))
    .body(json!({}).to_string());
  let mut response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
  HTTP/1.1 403 .
  Content-Type: application/problem+json
  Server: Rocket
  "###);
  assert_json_snapshot!(build_json_body(&mut response), @r###"
  {
    "status": 403,
    "title": "You are not permitted to perform this request",
    "type": "tag:universe,2020:problems/authentication/forbidden"
  }
  "###);
}

#[test]
fn test_patch_known_user_no_differences() {
  let service = ServiceWrapper::default();
  let user = User {
    user_id: uuid::Uuid::parse_str("2fcc3850-bb9b-405e-bbab-22978283fef8").unwrap(),
    username: "testuser".to_owned(),
    email: "testing@example.com".to_owned(),
    display_name: "Test User".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  let req = service
    .patch("/users/2fcc3850-bb9b-405e-bbab-22978283fef8")
    .header(ContentType::JSON)
    .header(authenticate_user(&service, &user))
    .body(json!({}).to_string());
  let mut response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
  HTTP/1.1 200 OK.
  Content-Type: application/json
  Server: Rocket
  "###);
  assert_json_snapshot!(build_json_body(&mut response), @r###"
  {
    "displayName": "Test User",
    "email": "testing@example.com",
    "id": "2fcc3850-bb9b-405e-bbab-22978283fef8",
    "username": "testuser"
  }
  "###);
}

#[test]
fn test_patch_known_user_with_differences() {
  let service = ServiceWrapper::default();
  let user = User {
    user_id: uuid::Uuid::parse_str("2fcc3850-bb9b-405e-bbab-22978283fef8").unwrap(),
    username: "testuser".to_owned(),
    email: "testing@example.com".to_owned(),
    display_name: "Test User".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  let req = service
    .patch("/users/2fcc3850-bb9b-405e-bbab-22978283fef8")
    .header(ContentType::JSON)
    .header(authenticate_user(&service, &user))
    .body(
      json!({
        "email": "new@example.com".to_owned(),
        "displayName": "New User".to_owned(),
      })
      .to_string(),
    );
  let mut response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
  HTTP/1.1 200 OK.
  Content-Type: application/json
  Server: Rocket
  "###);
  assert_json_snapshot!(build_json_body(&mut response), @r###"
  {
    "displayName": "New User",
    "email": "new@example.com",
    "id": "2fcc3850-bb9b-405e-bbab-22978283fef8",
    "username": "testuser"
  }
  "###);
}
