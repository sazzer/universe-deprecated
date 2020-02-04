use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;

/// Representation of a email address of some user in the system.
///
/// A email address is any valid UTF-8 string, but must not have any whitespace padding to either end.
#[derive(Debug, PartialEq, Clone, Serialize, FromSql)]
pub struct EmailAddress(String);

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implementation of the standard `FromStr` trait to allow us to parse any String into a `EmailAddress` object
impl FromStr for EmailAddress {
    type Err = EmailAddressParseError;

    /// Attempt to parse a string into a EmailAddress object.
    ///
    /// A email address is any valid UTF-8 string, but must not have any whitespace padding to either end.
    ///
    /// # Arguments
    /// * `s` The string to parse
    ///
    /// # Returns
    /// The result of parsing the email address. Either a `EmailAddress` object or an error if the incoming
    /// string was not valid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(EmailAddressParseError {})
        } else {
            Ok(EmailAddress(trimmed.to_owned()))
        }
    }
}

/// Allow us to pass `EmailAddress` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for EmailAddress {
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

/// Errors that can happen when parsing a string into a email address.
#[derive(Debug, PartialEq, Clone)]
pub struct EmailAddressParseError {}

impl std::fmt::Display for EmailAddressParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing email address")
    }
}

impl std::error::Error for EmailAddressParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use spectral::prelude::*;

    #[test]
    fn test_parse_valid_email_address() {
        let email_address: Result<EmailAddress, EmailAddressParseError> = "Test User".parse();
        assert_that(&email_address)
            .is_ok()
            .is_equal_to(EmailAddress("Test User".to_owned()));
    }
    #[test]
    fn test_parse_padded_email_address() {
        let email_address: Result<EmailAddress, EmailAddressParseError> = "  Test User  ".parse();
        assert_that(&email_address)
            .is_ok()
            .is_equal_to(EmailAddress("Test User".to_owned()));
    }
    #[test]
    fn test_parse_blank_email_address() {
        let email_address: Result<EmailAddress, EmailAddressParseError> = "   ".parse();

        assert_that(&email_address)
            .is_err()
            .is_equal_to(EmailAddressParseError {});
    }
    #[test]
    fn test_parse_empty_email_address() {
        let email_address: Result<EmailAddress, EmailAddressParseError> = "".parse();

        assert_that(&email_address)
            .is_err()
            .is_equal_to(EmailAddressParseError {});
    }

    #[test]
    fn test_serialize_valid_email_address() {
        let email_address = EmailAddress("Test User".parse().unwrap());

        let serialized = serde_json::to_value(email_address);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("Test User"));
    }
}
