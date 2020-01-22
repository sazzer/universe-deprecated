use serde::Serialize;
use std::str::FromStr;

/// Representation of a username of some user in the system.
///
/// A username is any valid UTF-8 string, but must not have any whitespace padding to either end.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Username(String);

/// Errors that can happen when parsing a string into a username.
#[derive(Debug, PartialEq, Clone)]
pub struct UsernameParseError {}

impl std::fmt::Display for UsernameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing username")
    }
}

impl std::error::Error for UsernameParseError {}

/// Implementation of the standard `FromStr` trait to allow us to parse any String into a `Username` object
impl FromStr for Username {
    type Err = UsernameParseError;

    /// Attempt to parse a string into a Username object.
    ///
    /// A username is any valid UTF-8 string, but must not have any whitespace padding to either end.
    ///
    /// # Arguments
    /// * `s` The string to parse
    ///
    /// # Returns
    /// The result of parsing the username. Either a `Username` object or an error if the incoming
    /// string was not valid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(UsernameParseError {})
        } else {
            Ok(Username(trimmed.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use spectral::prelude::*;

    #[test]
    fn test_parse_valid_username() {
        let username: Result<Username, UsernameParseError> = "testuser".parse();
        assert_that(&username)
            .is_ok()
            .is_equal_to(Username("testuser".to_owned()));
    }
    #[test]
    fn test_parse_padded_username() {
        let username: Result<Username, UsernameParseError> = "  testuser  ".parse();
        assert_that(&username)
            .is_ok()
            .is_equal_to(Username("testuser".to_owned()));
    }
    #[test]
    fn test_parse_blank_username() {
        let username: Result<Username, UsernameParseError> = "   ".parse();

        assert_that(&username)
            .is_err()
            .is_equal_to(UsernameParseError {});
    }
    #[test]
    fn test_parse_empty_username() {
        let username: Result<Username, UsernameParseError> = "".parse();

        assert_that(&username)
            .is_err()
            .is_equal_to(UsernameParseError {});
    }

    #[test]
    fn test_serialize_valid_username() {
        let username = Username("testuser".parse().unwrap());

        let serialized = serde_json::to_value(username);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("testuser"));
    }
}
