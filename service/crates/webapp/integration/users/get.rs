use crate::{build_headers, build_json_body, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot};
use test_env_log::test;
use universe_testdata::{seed, User};

#[test]
fn test_get_unknown_user() {
    let service = ServiceWrapper::default();

    let req = service.get("/users/2FCC3850-BB9B-405E-BBAB-22978283FEF8");
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
        user_id: uuid::Uuid::parse_str("2FCC3850-BB9B-405E-BBAB-22978283FEF8").unwrap(),
        username: "testuser".to_owned(),
        email: "testing@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..Default::default()
    };
    seed(service.database(), vec![&user]);

    let req = service.get("/users/2FCC3850-BB9B-405E-BBAB-22978283FEF8");
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
