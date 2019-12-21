use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;

/// Representation of a username of some user in the system.
///
/// A username is any valid UTF-8 string, but must not have any whitespace padding to either end.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Username(String);

/// Errors that can happen when parsing a string into a username.
#[derive(Debug, PartialEq, Clone)]
pub enum UsernameParseError {
    /// Error indicating that a parsed username was either empty or else was entirely whitespace.
    BlankUsername,
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
            Err(UsernameParseError::BlankUsername)
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
    fn to_sql(&self, t: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(VARCHAR, TEXT);
    to_sql_checked!();
}

/// Allow us to retrieve `Username` objects from Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be read directly as database
/// outputs without needing to construct it explicitly. Instead the Postgres crate will do so for us.
impl<'a> FromSql<'a> for Username {
    fn from_sql(t: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        String::from_sql(t, raw).map(Username)
    }
    accepts!(VARCHAR, TEXT);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use postgres::Error;
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
            .is_equal_to(UsernameParseError::BlankUsername);
    }
    #[test]
    fn test_parse_empty_username() {
        let username: Result<Username, UsernameParseError> = "".parse();

        assert_that(&username)
            .is_err()
            .is_equal_to(UsernameParseError::BlankUsername);
    }

    #[test]
    fn test_serialize_valid_username() {
        let username = Username("testuser".parse().unwrap());

        let serialized = serde_json::to_value(username);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("testuser"));
    }

    #[test]
    fn test_postgres_to_sql() {
        let database = TestDatabase::new();
        let username = Username("testuser".to_owned());
        let result = database.client().query("SELECT $1", &[&username]);

        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);

        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);

        let output_value: &str = rows.get(0).unwrap().get(0);
        assert_that(&output_value).is_equal_to("testuser");
    }

    #[test]
    fn test_postgres_from_sql_valid_type() {
        let database = TestDatabase::new();
        let result = database.client().query("SELECT $1", &[&"testuser"]);

        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);

        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);

        let output_value: Username = rows.get(0).unwrap().get(0);
        assert_that(&output_value).is_equal_to(Username("testuser".to_owned()));
    }

    #[test]
    fn test_postgres_from_sql_invalid_type() {
        let database = TestDatabase::new();
        let result = database.client().query("SELECT $1", &[&1]);
        let err: Error = result.err().unwrap();
        let err_msg = format!("{}", err);
        assert_that(&err_msg)
                        .is_equal_to("error serializing parameter 0: cannot convert between the Rust type `i32` and the Postgres type `text`".to_owned());
    }
}
