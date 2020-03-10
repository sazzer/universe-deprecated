use crate::model::*;
use std::boxed::Box;

/// The User Service to allow interactoins with user entities
pub trait UserService: Send + Sync {
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

    /// Register a new user
    ///
    /// # Arguments
    /// * `user` The user data to create the user from
    ///
    /// # Returns
    /// The user that was persisted
    fn register_user(&self, user: UserData) -> Result<UserEntity, RegisterUserError>;

    /// Update an existing user.
    ///
    /// This will load the user by ID, and then call a provided callback to mutate the user before persisting
    /// the changes back to the database
    ///
    /// # Arguments
    /// * `user_id` The ID of the user to update
    /// * `updater` The callback to mutate the user with
    ///
    /// # Returns
    /// The newly updated user
    fn update_user(
        &self,
        user_id: &UserID,
        updater: &mut dyn FnMut(UserData) -> Result<UserData, Box<dyn std::error::Error>>,
    ) -> Result<UserEntity, UpdateUserError>;
}

/// Enumeration of potential validation errors when creating a new user
#[derive(Debug, PartialEq)]
pub enum UserValidationError {
    DuplicateEmail,
    DuplicateUsername,
}

/// Enumeration of reasons why we failed to register a new user
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RegisterUserError {
    #[error("User details were invalid: {0:?}")]
    ValidationError(Vec<UserValidationError>),
    #[error("An unknown error occurred")]
    UnknownError,
}

/// Enumeration of reasons why we failed to update an existing user
#[derive(Debug, thiserror::Error)]
pub enum UpdateUserError {
    #[error("User details were invalid: {0:?}")]
    ValidationError(Vec<UserValidationError>),
    #[error("The user was not found")]
    UnknownUser,
    #[error("The version of the user record did not match")]
    OptimisticLockFailure,
    #[error("An error occurred updating the user details: {0}")]
    UpdateError(Box<dyn std::error::Error>),
    #[error("An unknown error occurred")]
    UnknownError,
}
