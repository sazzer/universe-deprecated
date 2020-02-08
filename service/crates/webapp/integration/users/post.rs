use crate::{build_headers, build_json_body, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot};
use rocket::http::ContentType;
use serde_json::json;

#[test]
fn test_post_empty_object() {
    let service = ServiceWrapper::default();

    let req = service
        .client()
        .post("/users")
        .header(ContentType::JSON)
        .body(json!({}).to_string());
    let mut response = req.dispatch();

    assert_snapshot!(build_headers(&response), @r###"
    HTTP/1.1 422 .
    Content-Type: application/problem+json
    Server: Rocket
    "###);
    assert_json_snapshot!(build_json_body(&mut response), @r###"
    {
      "errors": [
        {
          "field": "username",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "email",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "displayName",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "password",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        }
      ],
      "status": 422,
      "title": "The input had validation errors",
      "type": "tag:universe,2020:problems/validation-error"
    }
    "###);
}

#[test]
fn test_post_all_blank() {
    let service = ServiceWrapper::default();

    let req = service
        .client()
        .post("/users")
        .header(ContentType::JSON)
        .body(
            json!({
                "username": "",
                "displayName": "",
                "email": "",
                "password": ""
            })
            .to_string(),
        );
    let mut response = req.dispatch();

    assert_snapshot!(build_headers(&response), @r###"
    HTTP/1.1 422 .
    Content-Type: application/problem+json
    Server: Rocket
    "###);
    assert_json_snapshot!(build_json_body(&mut response), @r###"
    {
      "errors": [
        {
          "field": "username",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "email",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "displayName",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "password",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        }
      ],
      "status": 422,
      "title": "The input had validation errors",
      "type": "tag:universe,2020:problems/validation-error"
    }
    "###);
}

#[test]
fn test_post_all_whitespace() {
    let service = ServiceWrapper::default();

    let req = service
        .client()
        .post("/users")
        .header(ContentType::JSON)
        .body(
            json!({
                "username": "   ",
                "displayName": "   ",
                "email": "   ",
                "password": "   "
            })
            .to_string(),
        );
    let mut response = req.dispatch();

    assert_snapshot!(build_headers(&response), @r###"
    HTTP/1.1 422 .
    Content-Type: application/problem+json
    Server: Rocket
    "###);
    assert_json_snapshot!(build_json_body(&mut response), @r###"
    {
      "errors": [
        {
          "field": "username",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "email",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        },
        {
          "field": "displayName",
          "title": "Required field was missing a value",
          "type": "tag:universe,2020:validation-errors/missing"
        }
      ],
      "status": 422,
      "title": "The input had validation errors",
      "type": "tag:universe,2020:problems/validation-error"
    }
    "###);
}

#[test]
fn test_post_malformed_email() {
    let service = ServiceWrapper::default();

    let req = service
        .client()
        .post("/users")
        .header(ContentType::JSON)
        .body(
            json!({
                "username": "testuser",
                "displayName": "Test User",
                "email": "testuser",
                "password": "Pa55word"
            })
            .to_string(),
        );
    let mut response = req.dispatch();

    assert_snapshot!(build_headers(&response), @r###"
    HTTP/1.1 422 .
    Content-Type: application/problem+json
    Server: Rocket
    "###);
    assert_json_snapshot!(build_json_body(&mut response), @r###"
    {
      "errors": [
        {
          "field": "email",
          "title": "Email Address was malformed",
          "type": "tag:universe,2020:users/validation-errors/email/malformed"
        }
      ],
      "status": 422,
      "title": "The input had validation errors",
      "type": "tag:universe,2020:problems/validation-error"
    }
    "###);
}
