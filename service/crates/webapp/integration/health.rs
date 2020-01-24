use crate::{build_headers, build_json_body, ServiceWrapper};
use insta::{assert_json_snapshot, assert_snapshot};

#[test]
fn test_health() {
    let service = ServiceWrapper::default();

    let req = service.client().get("/health");
    let mut response = req.dispatch();

    assert_snapshot!(build_headers(&response), @r###"
    200
    Content-Type: application/json
    Server: Rocket
    "###);
    assert_json_snapshot!(build_json_body(&mut response), @r###"
    {
      "components": {
        "database": {
          "message": "Ok",
          "status": "OK"
        }
      },
      "status": "OK"
    }
    "###);
}
