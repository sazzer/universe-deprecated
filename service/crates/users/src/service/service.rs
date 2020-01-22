use super::repository::UserRepository;
use crate::model::*;
use tracing::warn;

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
}

/// The User Service to allow interactoins with user entities
pub struct UserServiceImpl<Repo> {
    repository: Repo,
}

/// Create a new User Service
///
/// # Arguments
/// * `repository` The User Reposutory to work in terms of
///
/// # Returns
/// The User Service
pub fn new_user_service<Repo: UserRepository + Send + Sync>(repository: Repo) -> impl UserService {
    UserServiceImpl { repository }
}

impl<Repo: UserRepository + Send + Sync> UserService for UserServiceImpl<Repo> {
    fn get_user_by_id(&self, user_id: &UserID) -> Option<UserEntity> {
        let user = self.repository.get_user_by_id(user_id);

        if user.is_none() {
            warn!("No user found with ID {}", user_id);
        }

        user
    }
}
