use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;

/// Representation of a display name of some user in the system.
///
/// A display name is any valid UTF-8 string, but must not have any whitespace padding to either end.
#[derive(Debug, PartialEq, Clone, Serialize, FromSql)]
pub struct DisplayName(String);

impl std::fmt::Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implementation of the standard `FromStr` trait to allow us to parse any String into a `DisplayName` object
impl FromStr for DisplayName {
    type Err = DisplayNameParseError;

    /// Attempt to parse a string into a DisplayName object.
    ///
    /// A display name is any valid UTF-8 string, but must not have any whitespace padding to either end.
    ///
    /// # Arguments
    /// * `s` The string to parse
    ///
    /// # Returns
    /// The result of parsing the display name. Either a `DisplayName` object or an error if the incoming
    /// string was not valid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(DisplayNameParseError {})
        } else {
            Ok(DisplayName(trimmed.to_owned()))
        }
    }
}

/// Allow us to pass `DisplayName` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for DisplayName {
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

/// Errors that can happen when parsing a string into a display name.
#[derive(Debug, PartialEq, Clone)]
pub struct DisplayNameParseError {}

impl std::fmt::Display for DisplayNameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing display name")
    }
}

impl std::error::Error for DisplayNameParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use spectral::prelude::*;

    #[test]
    fn test_parse_valid_display_name() {
        let display_name: Result<DisplayName, DisplayNameParseError> = "Test User".parse();
        assert_that(&display_name)
            .is_ok()
            .is_equal_to(DisplayName("Test User".to_owned()));
    }
    #[test]
    fn test_parse_padded_display_name() {
        let display_name: Result<DisplayName, DisplayNameParseError> = "  Test User  ".parse();
        assert_that(&display_name)
            .is_ok()
            .is_equal_to(DisplayName("Test User".to_owned()));
    }
    #[test]
    fn test_parse_blank_display_name() {
        let display_name: Result<DisplayName, DisplayNameParseError> = "   ".parse();

        assert_that(&display_name)
            .is_err()
            .is_equal_to(DisplayNameParseError {});
    }
    #[test]
    fn test_parse_empty_display_name() {
        let display_name: Result<DisplayName, DisplayNameParseError> = "".parse();

        assert_that(&display_name)
            .is_err()
            .is_equal_to(DisplayNameParseError {});
    }

    #[test]
    fn test_serialize_valid_display_name() {
        let display_name = DisplayName("Test User".parse().unwrap());

        let serialized = serde_json::to_value(display_name);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("Test User"));
    }
}
