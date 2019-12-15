use super::Password;
use crate::entity::Entity;
#[cfg(test)]
use chrono::{TimeZone, Utc};
use std::convert::AsRef;
use std::str::FromStr;
use uuid::Uuid;

/// Typesafe wrapper around a User ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserID(pub Uuid);

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
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "" => Err(ParseUsernameError::BlankUsername),
            username => Ok(Username(username.to_owned())),
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Enumeration of possible errors for parsing a username
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseUsernameError {
    BlankUsername,
}

#[cfg(test)]
impl Default for UserEntity {
    fn default() -> UserEntity {
        UserEntity {
            identity: crate::entity::Identity {
                id: UserID(uuid::Uuid::default()),
                version: uuid::Uuid::default(),
                created: Utc.ymd(2018, 1, 26).and_hms_micro(18, 30, 9, 453_829),
                updated: Utc.ymd(2018, 1, 26).and_hms_micro(18, 30, 9, 453_829),
            },
            data: UserData {
                username: Username("testuser".to_owned()),
                display_name: "Test User".to_owned(),
                email: "test@example.com".to_owned(),
                password: Password::from_hash(""),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use speculate::speculate;

    speculate! {
        describe "parse" {
            it "Parses a simple username" {
                let username: Username = "testuser".parse().unwrap();
                assert_eq!(Username("testuser".to_owned()), username);
            }
            it "Trims whitespace off of the username" {
                let username: Username = "  testuser  ".parse().unwrap();
                assert_eq!(Username("testuser".to_owned()), username);
            }
            it "Fails if the username is entirely whitespace" {
                let username: Result<Username, ParseUsernameError> = "   ".parse();
                assert_eq!(Err(ParseUsernameError::BlankUsername), username);
            }
            it "Fails if the username is the empty string" {
                let username: Result<Username, ParseUsernameError> = "".parse();
                assert_eq!(Err(ParseUsernameError::BlankUsername), username);
            }
        }
    }
}
