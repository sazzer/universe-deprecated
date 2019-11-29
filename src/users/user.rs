use super::Password;
use crate::entity::Entity;
use std::str::FromStr;
use uuid::Uuid;

/// Typesafe wrapper around a User ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserID(Uuid);

/// Typesafe wrapper around a username
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(pub String);

/// Data that makes up a single user
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserData {
    pub username: Username,
    pub email: String,
    pub display_name: String,
    pub password: Password,
}

/// Type that represents a user entity
pub type UserEntity = Entity<UserID, UserData>;

/// Implementation of `FromStr` for the `Username` struct so that we can easily parse a username
/// into a typesafe struct
impl FromStr for Username {
    /// The type of error to use
    type Err = ParseUsernameError;

    /// Attempt to parse the given string into a `Username` object
    ///
    /// # Arguments
    /// * `s` The string to parse
    ///
    /// # Returns
    /// The result of parsing the string into a Username object.
    /// If the input was either empty or only whitespace characters then this is an error.
    /// Otherwise we will trim the username and use that.
    ///
    /// # Examples
    /// ```
    /// # use universe::users::Username;
    /// let username: Username = "testuser".parse().unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "" => Err(ParseUsernameError::BlankUsername),
            username => Ok(Username(username.to_owned())),
        }
    }
}

/// Enumeration of possible errors for parsing a username
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseUsernameError {
    BlankUsername,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_username() {
        let username: Username = "testuser".parse().unwrap();
        assert_eq!(Username("testuser".to_owned()), username);
    }

    #[test]
    fn test_parse_username_trimmed() {
        let username: Username = "  testuser  ".parse().unwrap();
        assert_eq!(Username("testuser".to_owned()), username);
    }

    #[test]
    fn test_parse_username_whitespace_only() {
        let username: Result<Username, ParseUsernameError> = "   ".parse();
        assert_eq!(Err(ParseUsernameError::BlankUsername), username);
    }

    #[test]
    fn test_parse_username_empty_string() {
        let username: Result<Username, ParseUsernameError> = "".parse();
        assert_eq!(Err(ParseUsernameError::BlankUsername), username);
    }
}
