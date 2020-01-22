use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

/// Representation of a User ID of some user in the system.
///
/// A User ID is any valid UUID.
#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub struct UserID(Uuid);

/// Errors that can happen when parsing a string into a User ID.
#[derive(Debug, PartialEq, Clone)]
pub struct UserIDParseError {
    message: String,
}

impl std::fmt::Display for UserIDParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UserIDParseError {}

impl From<uuid::Error> for UserIDParseError {
    fn from(e: uuid::Error) -> Self {
        UserIDParseError {
            message: format!("Error parsing User ID: {}", e),
        }
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
    #[allow(unused)]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UserID(uuid)
    }
}

impl std::fmt::Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

#[cfg(test)]
mod tests {
    use super::*;
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
            .is_equal_to(UserIDParseError {
                message: "Error parsing User ID: invalid length: expected one of [36, 32], found 0"
                    .to_owned(),
            });
    }

    #[test]
    fn test_parse_blank_string() {
        let user_id: Result<UserID, UserIDParseError> = "     ".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError {
                message: "Error parsing User ID: invalid length: expected one of [36, 32], found 0"
                    .to_owned(),
            });
    }

    #[test]
    fn test_parse_invalid_string_bad_length() {
        let user_id: Result<UserID, UserIDParseError> = "non-uuid".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError {
                message: "Error parsing User ID: invalid length: expected one of [36, 32], found 8"
                    .to_owned(),
            });
    }

    #[test]
    fn test_parse_invalid_string_bad_character() {
        let user_id: Result<UserID, UserIDParseError> =
            "C37837C7-3E8C-4235-8A00-0845F598D12Z".parse();

        assert_that(&user_id)
            .is_err()
            .is_equal_to(UserIDParseError {
                message: "Error parsing User ID: invalid character: expected an optional prefix of `urn:uuid:` followed by 0123456789abcdefABCDEF-, found Z at 35"
                    .to_owned(),
            });
    }

    #[test]
    fn test_serialize_valid_user_id() {
        let user_id = UserID("f2c55656-d7a1-4e41-a311-fe653b9b15de".parse().unwrap());

        let serialized = serde_json::to_value(user_id);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(json!("f2c55656-d7a1-4e41-a311-fe653b9b15de"));
    }
}
