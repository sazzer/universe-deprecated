use super::Problem;
use std::collections::HashMap;
use universe_entity::ParseSortsError;

/// Construct a Problem that represents something unexpected went wrong
///
/// # Returns
/// The problem
pub fn unexpected_error() -> Problem {
    Problem {
        r#type: "tag:universe,2020:problems/unexpected-error".to_owned(),
        title: "An unexpected error occurred".to_owned(),
        status: 500,
        ..Default::default()
    }
}

/// Construct a Problem that indicates some sort fields were requested that couldn't be handled
pub fn invalid_sort_fields(error: ParseSortsError) -> Problem {
    match error {
        ParseSortsError::UnknownFields(fields) => {
            let mut extra = HashMap::new();
            extra.insert(
                "unknownFields".to_owned(),
                serde_json::to_value(fields).unwrap(),
            );

            Problem {
                r#type: "tag:universe,2020:problems/unknown-sort-fields".to_owned(),
                title: "Unknown Sort Fields requested".to_owned(),
                status: 422,
                extra,
                ..Default::default()
            }
        }
    }
}
