use super::{UserData, UserEntity, UserID, Username};

pub mod implementation;
pub mod repository;

/// Errors returned from a failed attempt to register a new user
pub enum UserRegistrationError {
    DuplicateEmailAddress,
}

/// Trait describing what can be achieved using the User Service
pub trait UserService: Send + Sync {
    /// Find the user that has the given ID
    ///
    /// # Arguments
    /// * `user_id` The user ID
    ///
    /// # Returns
    /// The user, if found. None if the user doesn't exist
    fn get_user_by_id(&self, user_id: &UserID) -> Option<UserEntity>;

    /// Determine if a given username already exists or not
    ///
    /// # Arguments
    /// * `username` The username to look up
    ///
    /// # Returns
    /// True if the username already exists. False if not.
    fn username_exists(&self, username: &Username) -> bool;

    /// Attempt to register a new user with the system
    ///
    /// # Arguments
    /// * `user` The details of the user to register
    ///
    /// # Returns
    /// The result of registering the user
    fn register_user(&self, user: UserData) -> Result<UserEntity, Vec<UserRegistrationError>>;
}
