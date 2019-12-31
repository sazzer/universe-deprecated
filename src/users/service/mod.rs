use super::{UserEntity, UserID};

pub mod implementation;
pub mod repository;

/// Trait describing what can be achieved using the User Service
pub trait UserService {
    /// Find the user that has the given ID
    ///
    /// # Arguments
    /// * `user_id` The user ID
    ///
    /// # Returns
    /// The user, if found. None if the user doesn't exist
    fn get_user_by_id(&self, user_id: UserID) -> Option<UserEntity>;
}
