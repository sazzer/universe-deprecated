use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    request::{FromRequest, Outcome},
    Data, Request, Response,
};
use tracing::debug;
use uuid::Uuid;

/// Fairing that will ensure a request has a Request ID attached to it, and that it is then copied
/// on to the Response on the way out again
pub struct RequestIdFairing {}

/// The name of the header to use for the Request ID
const REQUEST_ID_HEADER: &'static str = "X-Request-ID";

impl Fairing for RequestIdFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    /// Generate a Request ID and attach it to the request, if and only if there wasn't already one
    /// provided by the client
    fn on_request(&self, request: &mut Request, _: &Data) {
        if request.headers().get_one(REQUEST_ID_HEADER) == None {
            debug!("Adding Request ID to Request");
            let new_request_id = Uuid::new_v4().to_string();
            request.add_header(Header::new(REQUEST_ID_HEADER, new_request_id));
        }
    }

    /// Copy the Request ID from the Request onto the Response
    fn on_response(&self, request: &Request, response: &mut Response) {
        if let Some(request_id) = request.headers().get_one(REQUEST_ID_HEADER) {
            debug!("Adding Request ID to Response");
            response.set_header(Header::new(REQUEST_ID_HEADER, request_id.to_owned()));
        }
    }
}

/// Request Guard that represents the Request ID
/// Include this as a parameter in an instrumented handler and the Request ID will be logged all the way down
#[derive(Debug, PartialEq)]
pub struct RequestId(String);

impl<'a, 'r> FromRequest<'a, 'r> for RequestId {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one(REQUEST_ID_HEADER) {
            Some(request_id) => Outcome::Success(RequestId(request_id.to_owned())),
            None => Outcome::Forward(()),
        }
    }
}
