use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;

/// Representation of a username of some user in the system.
///
/// A username is any valid UTF-8 string, but must not have any whitespace padding to either end.
#[derive(Debug, PartialEq, Clone, Serialize, FromSql)]
pub struct Username(String);

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
            Err(UsernameParseError::Blank)
        } else {
            Ok(Username(trimmed.to_owned()))
        }
    }
}

/// Allow us to pass `Username` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for Username {
    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(VARCHAR, TEXT);
    to_sql_checked!();
}

/// Errors that can happen when parsing a string into a username.
#[derive(Debug, PartialEq, Clone)]
pub enum UsernameParseError {
    Blank,
}

impl std::fmt::Display for UsernameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing username")
    }
}

impl std::error::Error for UsernameParseError {}

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
            .is_equal_to(UsernameParseError::Blank);
    }
    #[test]
    fn test_parse_empty_username() {
        let username: Result<Username, UsernameParseError> = "".parse();

        assert_that(&username)
            .is_err()
            .is_equal_to(UsernameParseError::Blank);
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
