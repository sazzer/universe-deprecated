use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

/// Representation of a World ID of some world in the system.
///
/// A World ID is any valid UUID.
#[derive(Debug, PartialEq, Clone, Serialize, FromSql)]
pub struct WorldID(Uuid);

/// Errors that can happen when parsing a string into a World ID.
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum WorldIDParseError {
    #[error("World ID was malformed: {0}")]
    Malformed(#[from] uuid::Error),
}

impl WorldID {
    /// Construct a World ID from a UUID value
    ///
    /// # Arguments
    /// * `uuid` The UUID to use
    ///
    /// # Returns
    /// The World ID
    #[allow(unused)]
    pub fn from_uuid(uuid: Uuid) -> Self {
        WorldID(uuid)
    }
}

impl std::fmt::Display for WorldID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for WorldID {
    fn default() -> Self {
        WorldID(Uuid::new_v4())
    }
}
/// Implementation of the standard `FromStr` trait to allow us to parse any String into a `WorldID` object
impl FromStr for WorldID {
    type Err = WorldIDParseError;

    /// Attempt to parse a string into a WorldID object.
    ///
    /// A World ID is any valid UUID.
    ///
    /// # Arguments
    /// * `s` The string to parse
    ///
    /// # Returns
    /// The result of parsing the World ID. Either a `WorldID` object or an error if the incoming
    /// string was not valid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid: Uuid = s.trim().parse()?;
        Ok(WorldID(uuid))
    }
}

/// Allow us to pass `WorldID` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for WorldID {
    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(UUID);
    to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::*;
    use serde_json::json;
    use spectral::prelude::*;
    use test_env_log::test;

    #[test]
    fn test_parse_valid_world_id() {
        let world_id: Result<WorldID, WorldIDParseError> =
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse();

        assert_that(&world_id).is_ok().is_equal_to(WorldID(
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
        ));
    }

    #[test]
    fn test_parse_padded_world_id() {
        let world_id: Result<WorldID, WorldIDParseError> =
            "  f2c55656-d7a1-4e41-a311-fe653b9b15de    ".parse();

        assert_that(&world_id).is_ok().is_equal_to(WorldID(
            "f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap(),
        ));
    }

    #[test]
    fn test_parse_empty_string() {
        let world_id: Result<WorldID, WorldIDParseError> = "".parse();

        assert_matches!(world_id.unwrap_err(), WorldIDParseError::Malformed(_));
    }

    #[test]
    fn test_parse_blank_string() {
        let world_id: Result<WorldID, WorldIDParseError> = "     ".parse();

        assert_matches!(world_id.unwrap_err(), WorldIDParseError::Malformed(_));
    }

    #[test]
    fn test_parse_invalid_string_bad_length() {
        let world_id: Result<WorldID, WorldIDParseError> = "non-uuid".parse();

        assert_matches!(world_id.unwrap_err(), WorldIDParseError::Malformed(_));
    }

    #[test]
    fn test_parse_invalid_string_bad_character() {
        let world_id: Result<WorldID, WorldIDParseError> =
            "C37837C7-3E8C-4235-8A00-0845F598D12Z".parse();

        assert_matches!(world_id.unwrap_err(), WorldIDParseError::Malformed(_));
    }

    #[test]
    fn test_serialize_valid_world_id() {
        let world_id = WorldID("f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap());

        let serialized = serde_json::to_value(world_id);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("f2c55656-d7a1-4e41-a311-fe653b9b15de"));
    }
}
