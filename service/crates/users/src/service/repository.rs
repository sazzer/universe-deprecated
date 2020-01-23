use crate::{UserEntity, UserID};
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
}
