use super::healthchecker::Healthchecker;
use crate::request_id::RequestId;
use rocket::{get, http::Status, response::status, routes, Route, State};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::collections::HashMap;

/// Enumeration of the possible status codes for a health check
#[derive(Debug, Serialize)]
enum HealthcheckStatus {
    /// The healthcheck was a success
    OK,
    /// The healthcheck was a failure
    FAILURE,
}

/// Representation of the healthcheck status for a single component
#[derive(Debug, Serialize)]
struct HealthcheckComponent {
    status: HealthcheckStatus,
    message: String,
}

/// Representation of the healthcheck status for the entire system
#[derive(Debug, Serialize)]
struct HealthcheckResponse {
    status: HealthcheckStatus,
    components: HashMap<String, HealthcheckComponent>,
}

impl HealthcheckResponse {
    /// Helper to get the HTTP status code for the healthcheck
    fn get_status_code(&self) -> Status {
        match self.status {
            HealthcheckStatus::OK => Status::Ok,
            HealthcheckStatus::FAILURE => Status::InternalServerError,
        }
    }
}

#[get("/health")]
#[tracing::instrument(skip(healthchecker))]
fn get_health(
    _request_id: RequestId,
    healthchecker: State<Healthchecker>,
) -> status::Custom<Json<HealthcheckResponse>> {
    let health = healthchecker.check_health();

    let mut status: HealthcheckStatus = HealthcheckStatus::OK;
    let mut components = HashMap::new();
    for (k, v) in health.iter() {
        match v {
            Err(message) => {
                status = HealthcheckStatus::FAILURE;
                components.insert(
                    k.clone(),
                    HealthcheckComponent {
                        status: HealthcheckStatus::FAILURE,
                        message: message.clone(),
                    },
                );
            }
            Ok(message) => {
                components.insert(
                    k.clone(),
                    HealthcheckComponent {
                        status: HealthcheckStatus::OK,
                        message: message.clone(),
                    },
                );
            }
        }
    }

    let response = HealthcheckResponse { status, components };

    status::Custom(response.get_status_code(), Json(response))
}

pub fn routes() -> Vec<Route> {
    routes![get_health]
}
