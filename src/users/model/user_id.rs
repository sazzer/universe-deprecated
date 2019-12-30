use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::error::Error;
use std::str::FromStr;
use uuid::Uuid;

/// Representation of a User ID of some user in the system.
///
/// A User ID is any valid UUID.
#[derive(Debug, PartialEq, Clone, Serialize, FromSql)]
pub struct UserID(Uuid);

/// Errors that can happen when parsing a string into a User ID.
#[derive(Debug, PartialEq, Clone)]
pub struct UserIDParseError {}

impl std::fmt::Display for UserIDParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing User ID")
    }
}

impl std::error::Error for UserIDParseError {}

impl From<uuid::Error> for UserIDParseError {
    fn from(_: uuid::Error) -> Self {
        UserIDParseError {}
    }
}

impl UserID {
    /// Construct a User ID from a UUID value
    ///
    /// # Arguments
    /// * `uuid` The UUID to use
    ///
    /// # Returns
    /// The User ID
    pub fn from_uuid(uuid: Uuid) -> Self {
        UserID(uuid)
    }
}
/// Implementation of the standard `FromStr` trait to allow us to parse any String into a `UserID` object
impl FromStr for UserID {
    type Err = UserIDParseError;

    /// Attempt to parse a string into a UserID object.
    ///
    /// A User ID is any valid UUID.
    ///
    /// # Arguments
    /// * `s` The string to parse
    ///
    /// # Returns
    /// The result of parsing the User ID. Either a `UserID` object or an error if the incoming
    /// string was not valid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid: Uuid = s.trim().parse()?;
        Ok(UserID(uuid))
    }
}

/// Allow us to pass `UserID` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for UserID {
    fn to_sql(&self, t: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(UUID);
    to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use postgres::Error;
    use serde_json::json;
    use spectral::prelude::*;
    use test_env_log::test;

    #[test]
    fn test_parse_valid_user_id() {
        let user_id: Result<UserID, UserIDParseError> =
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse();

        assert_that(&user_id).is_ok().is_equal_to(UserID(
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
        ));
    }

    #[test]
    fn test_parse_padded_user_id() {
        let user_id: Result<UserID, UserIDParseError> =
            "  f2c55656-d7a1-4e41-a311-fe653b9b15de    ".parse();

        assert_that(&user_id).is_ok().is_equal_to(UserID(
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
        ));
    }

    #[test]
    fn test_parse_empty_string() {
        let user_id: Result<UserID, UserIDParseError> = "".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError {});
    }

    #[test]
    fn test_parse_blank_string() {
        let user_id: Result<UserID, UserIDParseError> = "     ".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError {});
    }

    #[test]
    fn test_parse_invalid_string() {
        let user_id: Result<UserID, UserIDParseError> = "non-uuid".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError {});
    }

    #[test]
    fn test_serialize_valid_user_id() {
        let user_id = UserID("f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap());

        let serialized = serde_json::to_value(user_id);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("f2c55656-d7a1-4e41-a311-fe653b9b15de"));
    }

    #[test]
    fn test_postgres_to_sql() {
        let database = TestDatabase::new();
        let uuid: Uuid = "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap();
        let user_id = UserID(uuid);
        let result = database.client().query("SELECT $1::uuid", &[&user_id]);
        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);
        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);
        let output_value: Uuid = rows.get(0).unwrap().get(0);
        assert_that(&output_value).is_equal_to(uuid);
    }

    #[test]
    fn test_postgres_from_sql_valid_type() {
        let database = TestDatabase::new();
        let uuid: Uuid = "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap();
        let result = database.client().query("SELECT $1::uuid", &[&uuid]);
        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);
        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);
        let output_value: UserID = rows.get(0).unwrap().get(0);
        assert_that(&output_value).is_equal_to(UserID(uuid));
    }

    #[test]
    fn test_postgres_from_sql_invalid_type() {
        let database = TestDatabase::new();
        let result = database.client().query("SELECT 1", &[]);
        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);

        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);

        let output_value: Result<UserID, Error> = row.try_get(0);
        assert_that(&output_value).is_err();
        let err_msg = format!("{}", output_value.unwrap_err());
        assert_that(&err_msg)
                        .is_equal_to("error deserializing column 0: cannot convert between the Rust type `universe::users::model::user_id::UserID` and the Postgres type `int4`".to_owned());
    }
}
