use crate::{build_headers, build_json_body, build_rewrite_headers, regex_replace, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot, dynamic_redaction};
use rocket::http::ContentType;
use serde_json::json;
use spectral::prelude::*;
use test_env_log::test;
use universe_testdata::{seed, User};

#[test]
fn test_post_empty_object() {
  let service = ServiceWrapper::default();

  let req = service
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

  let req = service.post("/users").header(ContentType::JSON).body(
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

  let req = service.post("/users").header(ContentType::JSON).body(
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

  let req = service.post("/users").header(ContentType::JSON).body(
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

#[test]
fn test_post_duplicate_username() {
  let service = ServiceWrapper::default();

  let user = User {
    username: "testuser".to_owned(),
    email: "testing@example.com".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  let req = service.post("/users").header(ContentType::JSON).body(
    json!({
        "username": "testuser",
        "displayName": "Test User",
        "email": "other@example.com",
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
          "field": "username",
          "title": "The username is already registered",
          "type": "tag:universe,2020:users/validation-errors/username/duplicate"
        }
      ],
      "status": 422,
      "title": "The input had validation errors",
      "type": "tag:universe,2020:problems/validation-error"
    }
    "###);
}

#[test]
fn test_post_duplicate_email() {
  let service = ServiceWrapper::default();

  let user = User {
    username: "testuser".to_owned(),
    email: "testing@example.com".to_owned(),
    ..Default::default()
  };
  seed(service.database(), vec![&user]);

  let req = service.post("/users").header(ContentType::JSON).body(
    json!({
        "username": "other",
        "displayName": "Test User",
        "email": "testing@example.com",
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
          "title": "The email address is already registered",
          "type": "tag:universe,2020:users/validation-errors/email/duplicate"
        }
      ],
      "status": 422,
      "title": "The input had validation errors",
      "type": "tag:universe,2020:problems/validation-error"
    }
    "###);
}

#[test]
fn test_post_success() {
  let service = ServiceWrapper::default();

  let req = service.post("/users").header(ContentType::JSON).body(
    json!({
        "username": "testuser",
        "displayName": "Test User",
        "email": "testing@example.com",
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
        ".id" => "[uuid]",
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

#[test]
fn test_post_refetch() {
  let service = ServiceWrapper::default();

  let post_req = service.post("/users").header(ContentType::JSON).body(
    json!({
        "username": "testuser",
        "displayName": "Test User",
        "email": "testing@example.com",
        "password": "Pa55word"
    })
    .to_string(),
  );
  let mut post_response = post_req.dispatch();

  let parsed = build_json_body(&mut post_response);
  let new_id = parsed.get("id").unwrap().as_str().unwrap().to_owned();

  let req = service.client().get(format!("/users/{}", new_id));
  let mut response = req.dispatch();

  assert_snapshot!(build_rewrite_headers(&response, |h| {
    let h = regex_replace(h, r#"/users/[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}"#, "/users/d4ebcc15-ddf2-45e4-b263-892984b0e248");
    let h = regex_replace(h, r#"ETag: "[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}""#, r#"ETag: "a7fd01dc-dcf7-45dd-a932-0b6b263e17d0""#);
    regex_replace(h, r#"^Last-Modified: .*$"#, "Last-Modified: Wed, 11 Mar 2020 13:00:36 GMT")
  }), @r###"
  HTTP/1.1 200 OK.
  Content-Type: application/json
  Link: </users/d4ebcc15-ddf2-45e4-b263-892984b0e248>; rel="self"
  Accept-Patch: application/merge-patch+json
  ETag: "a7fd01dc-dcf7-45dd-a932-0b6b263e17d0"
  Last-Modified: Wed, 11 Mar 2020 13:00:36 GMT
  Cache-Control: public, max-age=3600
  Server: Rocket
  "###);
  assert_json_snapshot!(build_json_body(&mut response), {
        ".id" => dynamic_redaction(move |value, _| {
            assert_that(&value.as_str()).is_some().is_equal_to(new_id.as_str());
            "[uuid]"
        })
    },
    @r###"
    {
      "displayName": "Test User",
      "email": "testing@example.com",
      "id": "[uuid]",
      "username": "testuser"
    }
    "###);
}
