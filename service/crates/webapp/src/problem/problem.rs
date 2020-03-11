use rocket::{
    http::{ContentType, Status},
    response::{Responder, Response},
    Request,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// Struct representing an RFC-7807 problem returned by a REST API
#[derive(Serialize, Debug, PartialEq)]
pub struct Problem {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Default for Problem {
    /// Construct the default shape of a Problem, allowing other fields to be populated as needed
    fn default() -> Self {
        Self {
            r#type: "".to_owned(),
            title: "".to_owned(),
            status: 400,
            detail: None,
            instance: None,
            extra: HashMap::new(),
        }
    }
}

impl<'a> Responder<'a> for Problem {
    /// Generate a Rocket response for the Problem
    fn respond_to(self, req: &Request) -> Result<Response<'a>, Status> {
        Response::build()
            .merge(Json(&self).respond_to(req)?)
            .header(ContentType::new("application", "problem+json"))
            .raw_status(self.status, "")
            .ok()
    }
}
