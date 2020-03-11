use crate::{build_headers, build_json_body, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot};
use test_env_log::test;
use universe_testdata::{seed, User};

#[test]
fn test_get_unknown_user() {
  let service = ServiceWrapper::default();

  let req = service.get("/users/2fcc3850-bb9b-405e-bbab-22978283fef8");
  let mut response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
    HTTP/1.1 404 .
    Content-Type: application/problem+json
    Server: Rocket
    "###);
  assert_json_snapshot!(build_json_body(&mut response), @r###"
    {
      "status": 404,
      "title": "The requested user could not be found",
      "type": "tag:universe,2020:users/problems/unknown-user"
    }
    "###);
}

#[test]
fn test_get_known_user() {
  let service = ServiceWrapper::default();

  let user = User {
    user_id: uuid::Uuid::parse_str("2fcc3850-bb9b-405e-bbab-22978283fef8").unwrap(),
    version: uuid::Uuid::parse_str("301e8ef9-1077-43e1-abe1-6b8c743cdd1b").unwrap(),
    updated: "2020-03-11T12:54:25Z".parse().unwrap(),
    username: "testuser".to_owned(),
    email: "testing@example.com".to_owned(),
    display_name: "Test User".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  let req = service.get("/users/2fcc3850-bb9b-405e-bbab-22978283fef8");
  let mut response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
  HTTP/1.1 200 OK.
  Content-Type: application/json
  Link: </users/2fcc3850-bb9b-405e-bbab-22978283fef8>; rel="self"
  Accept-Patch: application/merge-patch+json
  ETag: "301e8ef9-1077-43e1-abe1-6b8c743cdd1b"
  Last-Modified: Wed, 11 Mar 2020 12:54:25 GMT
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
