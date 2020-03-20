use crate::{build_headers, build_json_body, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot};
use test_env_log::test;

#[test]
fn test_search_no_worlds() {
  let service = ServiceWrapper::default();

  let req = service.get("/worlds");
  let mut response = req.dispatch();

  assert_snapshot!(build_headers(&response), @r###"
  HTTP/1.1 200 OK.
  Content-Type: application/json
  Cache-Control: public, max-age=3600
  Server: Rocket
  "###);
  assert_json_snapshot!(build_json_body(&mut response), @r###"
  {
    "entries": [],
    "total": 0
  }
  "###);
}
