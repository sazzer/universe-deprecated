use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;

/// Representation of a slug of some world in the system.
///
/// A slug is any valid UTF-8 string, but must not have any whitespace padding to either end.
#[derive(Debug, PartialEq, Clone, Serialize, FromSql)]
pub struct Slug(String);

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Implementation of the standard `FromStr` trait to allow us to parse any String into a `Slug` object
impl FromStr for Slug {
    type Err = SlugParseError;

    /// Attempt to parse a string into a Slug object.
    ///
    /// A slug is any valid UTF-8 string, but must not have any whitespace padding to either end.
    ///
    /// # Arguments
    /// * `s` The string to parse
    ///
    /// # Returns
    /// The result of parsing the slug. Either a `Slug` object or an error if the incoming
    /// string was not valid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(SlugParseError::Blank)
        } else {
            Ok(Slug(trimmed.to_owned()))
        }
    }
}

/// Allow us to pass `Slug` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for Slug {
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

/// Errors that can happen when parsing a string into a slug.
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum SlugParseError {
    #[error("Slug was blank")]
    Blank,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use spectral::prelude::*;
    use test_env_log::test;

    #[test]
    fn test_parse_valid_slug() {
        let slug: Result<Slug, SlugParseError> = "testworld".parse();
        assert_that(&slug)
            .is_ok()
            .is_equal_to(Slug("testworld".to_owned()));
    }
    #[test]
    fn test_parse_padded_slug() {
        let slug: Result<Slug, SlugParseError> = "  testworld  ".parse();
        assert_that(&slug)
            .is_ok()
            .is_equal_to(Slug("testworld".to_owned()));
    }
    #[test]
    fn test_parse_blank_slug() {
        let slug: Result<Slug, SlugParseError> = "   ".parse();

        assert_that(&slug)
            .is_err()
            .is_equal_to(SlugParseError::Blank);
    }
    #[test]
    fn test_parse_empty_slug() {
        let slug: Result<Slug, SlugParseError> = "".parse();

        assert_that(&slug)
            .is_err()
            .is_equal_to(SlugParseError::Blank);
    }

    #[test]
    fn test_serialize_valid_slug() {
        let slug = Slug("testworld".parse().unwrap());

        let serialized = serde_json::to_value(slug);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("testworld"));
    }
}
