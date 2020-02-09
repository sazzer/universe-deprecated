use super::Problem;

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
