use crate::users::{UserEntity, Username};

/// Trait defining the ways that we can interact with user entities in the database
pub trait UserRepository {
    /// Find the user that has the given username
    ///
    /// # Arguments
    /// * `username` The user with the username
    ///
    /// # Returns
    /// The user, if found. None if the user doesn't exist
    fn get_user_by_username(&self, username: Username) -> Option<UserEntity>;
}
