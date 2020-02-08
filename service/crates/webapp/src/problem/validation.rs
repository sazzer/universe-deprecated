use super::Problem;
use serde::Serialize;
use std::collections::HashMap;

/// Struct representing a validation error to go into a Problem response
#[derive(Serialize, Debug)]
pub struct ValidationError {
    pub r#type: String,
    pub title: String,
    pub field: String,
}

/// Construct a Problem that represents validation errors with an incoming request
///
/// # Arguments
/// * `errors` The validation errors that we are representing
///
/// # Returns
/// The problem
pub fn validation_error(errors: Vec<ValidationError>) -> Problem {
    let mut extra_data = HashMap::new();

    extra_data.insert("errors".to_owned(), serde_json::to_value(errors).unwrap());

    Problem {
        r#type: "tag:universe,2020:problems/validation-error".to_owned(),
        title: "The input had validation errors".to_owned(),
        status: 422,
        extra: extra_data,
        ..Default::default()
    }
}

/// Construct a validation error to say that a field was missing from the request
///
/// # Arguments
/// * `field` The name of the field that was missing
///
/// # Returns
/// The validation error
pub fn missing_error(field: &str) -> ValidationError {
    ValidationError {
        r#type: "tag:universe,2020:validation-errors/missing".to_owned(),
        title: "Required field was missing a value".to_owned(),
        field: field.to_owned(),
    }
}
