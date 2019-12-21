use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{
    de,
    de::{Deserialize, Deserializer, Visitor},
    Serialize, Serializer,
};
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Representation of a User ID of some user in the system.
///
/// A User ID is any valid UUID.
#[derive(Debug, PartialEq, Clone)]
pub struct UserID(Uuid);

/// Errors that can happen when parsing a string into a User ID.
#[derive(Debug, PartialEq, Clone)]
pub enum UserIDParseError {
    InvalidUUID,
}

impl From<uuid::Error> for UserIDParseError {
    fn from(_: uuid::Error) -> Self {
        UserIDParseError::InvalidUUID
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

/// Allow us to retrieve `UserID` objects from Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be read directly as database
/// outputs without needing to construct it explicitly. Instead the Postgres crate will do so for us.
impl<'a> FromSql<'a> for UserID {
    fn from_sql(t: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Uuid::from_sql(t, raw).map(UserID)
    }
    accepts!(UUID);
}

/// Allow us to serialize `UserID` objects into Serde structures, so that they can be provided to
/// anything that works as such - for example, Tera - without needing to extract the value from inside.
impl Serialize for UserID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

/// Serde Visitor to allow us to parse a String value into a UserID object.
struct UserIDVisitor {}
impl<'de> Visitor<'de> for UserIDVisitor {
    type Value = UserID;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a non-blank string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        s.parse()
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
    }
}

/// Allow us to deserialize `UserID` objects from Serde structures, should we ever need to do so.
impl<'de> Deserialize<'de> for UserID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = UserIDVisitor {};
        deserializer.deserialize_string(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use postgres::Error;
    use serde_json::json;
    use spectral::prelude::*;

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
            .is_equal_to(UserIDParseError::InvalidUUID);
    }

    #[test]
    fn test_parse_blank_string() {
        let user_id: Result<UserID, UserIDParseError> = "     ".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError::InvalidUUID);
    }

    #[test]
    fn test_parse_invalid_string() {
        let user_id: Result<UserID, UserIDParseError> = "non-uuid".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError::InvalidUUID);
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
    fn test_deserialize_valid_user_id() {
        let result: Result<UserID, _> =
            serde_json::from_value(json!("f2c55656-d7a1-4e41-a311-fe653b9b15de"));
        assert_that(&result).is_ok().is_equal_to(UserID(
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
        ));
    }
    #[test]
    fn test_deserialize_padded_user_id() {
        let result: Result<UserID, _> =
            serde_json::from_value(json!("  f2c55656-d7a1-4e41-a311-fe653b9b15de  "));
        assert_that(&result).is_ok().is_equal_to(UserID(
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
        ));
    }
    #[test]
    fn test_deserialize_empty_user_id() {
        let result: Result<UserID, serde_json::Error> = serde_json::from_value(json!(""));
        assert_that(&result).is_err();

        let err = result.unwrap_err();
        let err_msg = format!("{}", err);
        assert_that(&err_msg)
            .is_equal_to("invalid value: string \"\", expected a non-blank string".to_owned());
    }
    #[test]
    fn test_deserialize_blank_user_id() {
        let result: Result<UserID, serde_json::Error> = serde_json::from_value(json!("    "));
        assert_that(&result).is_err();

        let err = result.unwrap_err();
        let err_msg = format!("{}", err);
        assert_that(&err_msg)
            .is_equal_to("invalid value: string \"    \", expected a non-blank string".to_owned());
    }
    #[test]
    fn test_deserialize_invalud_user_id() {
        let result: Result<UserID, serde_json::Error> = serde_json::from_value(json!("invalid"));
        assert_that(&result).is_err();

        let err = result.unwrap_err();
        let err_msg = format!("{}", err);
        assert_that(&err_msg).is_equal_to(
            "invalid value: string \"invalid\", expected a non-blank string".to_owned(),
        );
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
        let result = database.client().query("SELECT $1", &[&1]);
        let err: Error = result.err().unwrap();
        let err_msg = format!("{}", err);
        assert_that(&err_msg)
                    .is_equal_to("error serializing parameter 0: cannot convert between the Rust type `i32` and the Postgres type `text`".to_owned());
    }
}
