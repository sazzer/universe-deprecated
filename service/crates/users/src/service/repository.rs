use crate::{UserEntity, UserID, Username};
#[cfg(test)]
use mockall::automock;

/// Repository that describes how to access user data
#[cfg_attr(test, automock)]
pub trait UserRepository {
    /// Retrieve the user from the data store that has the given unique ID
    ///
    /// # Arguments
    /// * `user_id` The ID of the user to retrieve
    ///
    /// # Returns
    /// The user, or `None` if it wasn't found
    fn get_user_by_id(&self, user_id: &UserID) -> Option<UserEntity>;

    /// Retrieve the user from the data store that has the given unique Username
    ///
    /// # Arguments
    /// * `username` The Username of the user to retrieve
    ///
    /// # Returns
    /// The user, or `None` if it wasn't found
    fn get_user_by_username(&self, username: &Username) -> Option<UserEntity>;

    /// Create a new user record in the data store
    ///
    /// # Arguments
    /// * `user` The user entity to persist to the data store
    ///
    /// # Returns
    /// The user that was persisted
    fn create_user(&self, user: UserEntity) -> Result<UserEntity, PersistUserError>;

    /// Update an existing user in the data store
    ///
    /// # Arguments
    /// * `user` The user entity to persist to the data store
    ///
    /// # Returns
    /// The user that was persisted
    fn update_user(&self, user: UserEntity) -> Result<UserEntity, PersistUserError>;
}

/// Enumeration of reasons why we failed to persist a newly created user
#[derive(Debug, PartialEq)]
pub enum PersistUserError {
    DuplicateId,
    DuplicateUsername,
    DuplicateEmail,
    UnknownError,
}

impl std::fmt::Display for PersistUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PersistUserError::DuplicateId => "Duplicate User ID",
                PersistUserError::DuplicateUsername => "Duplicate Username",
                PersistUserError::DuplicateEmail => "Duplicate Email Address",
                PersistUserError::UnknownError => "An unknown error occurred",
            }
        )
    }
}

impl std::error::Error for PersistUserError {}
