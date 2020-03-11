use regex::Regex;
use rocket::local::LocalResponse;
use serde_json::from_slice;
use serde_json::Value;
use std::fmt::Write;

/// Simple wrapper around a regex used to perform a replacement on a string
pub fn regex_replace<I, R, O>(input: I, regex: R, replacement: O) -> String
where
    I: Into<String>,
    R: Into<String>,
    O: Into<String>,
{
    let re = Regex::new(&regex.into()).unwrap();
    re.replace_all(input.into().as_str(), replacement.into().as_str())
        .to_string()
}

/// Build the headers that have come out of a response so that they can be asserted on
pub fn build_headers(response: &LocalResponse) -> String {
    build_rewrite_headers(response, |h| h)
}

/// Build the headers that have come out of a response so that they can be asserted on.
/// This version allows the caller to re-write the headers on the way through, to account for dynamic values
pub fn build_rewrite_headers<F>(response: &LocalResponse, rewriter: F) -> String
where
    F: Fn(String) -> String,
{
    let mut output = String::new();
    writeln!(
        output,
        "HTTP/1.1 {} {}.", // Bit icky, but the "." means there's no trailing space
        response.status().code,
        response.status().reason
    )
    .unwrap();

    let mut headers = response.headers().clone();
    headers.remove("X-Request-ID");
    for header in headers.iter() {
        let header_str = format!("{}", header);
        writeln!(output, "{}", rewriter(header_str)).unwrap();
    }

    output
}

/// Parse the JSON body that has come out of a response so that it can be asserted
pub fn build_json_body(response: &mut LocalResponse) -> Value {
    let body = response.body_bytes().unwrap();
    from_slice(&body).unwrap()
}
