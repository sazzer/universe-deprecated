use crate::users::{UserEntity, UserID, Username};

/// Trait defining the ways that we can interact with user entities in the database
pub trait UserRepository {
    /// Find the user that has the given ID
    ///
    /// # Arguments
    /// * `user_id` The user ID
    ///
    /// # Returns
    /// The user, if found. None if the user doesn't exist
    fn get_user_by_id(&self, user_id: UserID) -> Option<UserEntity>;

    /// Find the user that has the given username
    ///
    /// # Arguments
    /// * `username` The username
    ///
    /// # Returns
    /// The user, if found. None if the user doesn't exist
    fn get_user_by_username(&self, username: Username) -> Option<UserEntity>;

    /// Find the user that has the given email address
    ///
    /// # Arguments
    /// * `email` The email address
    ///
    /// # Returns
    /// The user, if found. None if the user doesn't exist
    fn get_user_by_email(&self, email: String) -> Option<UserEntity>;
}
