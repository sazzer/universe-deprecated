use super::{UserEntity, Username};
use log::debug;
use mockall::automock;
use std::boxed::Box;

/// Trait to represent the User Service for interacting with users
#[automock]
pub trait UserService: Send + Sync {
    /// Check if a user with the given username already exists.
    ///
    /// # Arguments
    /// * `username` The username to look for
    ///
    /// # Returns
    /// True if the username is already known to the system. False if it's unknown.
    fn username_exists(&self, username: Username) -> bool;
}

/// Repository that can be used to work with user records in the underlying data store
#[automock]
pub trait UserRepository: Send + Sync {
    /// Load the user from the database with the given username.
    ///
    /// # Arguments
    /// * `username` The username to look for
    ///
    /// # Returns
    /// The user entity if it was found, or `None` if there was no match.
    fn get_by_username(&self, username: Username) -> Option<UserEntity>;
}

/// The actual user service implementation
pub struct UserServiceImpl {
    repository: Box<dyn UserRepository>,
}

impl UserServiceImpl {
    /// Create a new User Service
    ///
    /// # Arguments
    /// * `repository` The user repostory to use to access data
    ///
    /// # Returns
    /// The user service to work with
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        UserServiceImpl {
            repository: repository,
        }
    }
}

impl UserService for UserServiceImpl {
    fn username_exists(&self, username: Username) -> bool {
        debug!("Looking up username {:?}", username);

        self.repository.get_by_username(username).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::users::*;
    use speculate::speculate;

    speculate! {
        before {
            let mut user_repository = MockUserRepository::new();
        }

        describe "username_exists" {
            it "works when the user doesn't exist" {
                user_repository
                    .expect_get_by_username()
                    .with(mockall::predicate::eq(Username("testuser".to_owned())))
                    .times(1)
                    .return_const(None);

                let user_service = UserServiceImpl::new(Box::new(user_repository));

                let result = user_service.username_exists(Username("testuser".to_owned()));
                assert_eq!(false, result);
            }
            it "works when the user does exist" {
                user_repository
                    .expect_get_by_username()
                    .with(mockall::predicate::eq(Username("testuser".to_owned())))
                    .times(1)
                    .return_const(Some(UserEntity::default()));

                let user_service = UserServiceImpl::new(Box::new(user_repository));

                let result = user_service.username_exists(Username("testuser".to_owned()));
                assert_eq!(true, result);
            }

        }
    }
}
