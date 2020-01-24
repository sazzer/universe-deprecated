use rocket::local::LocalResponse;
use serde_json::from_slice;
use serde_json::Value;
use std::fmt::Write;

/// Build the headers that have come out of a response so that they can be asserted on
pub fn build_headers(response: &LocalResponse) -> String {
    let mut output = String::new();
    writeln!(output, "{}", response.status().code).unwrap();

    let headers = response.headers().clone();
    for header in headers.iter() {
        writeln!(output, "{}", header).unwrap();
    }

    output
}

/// Parse the JSON body that has come out of a response so that it can be asserted
pub fn build_json_body(response: &mut LocalResponse) -> Value {
    let body = response.body_bytes().unwrap();
    from_slice(&body).unwrap()
}
