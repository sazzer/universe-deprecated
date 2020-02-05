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

    /// Retrieve the user from the data store that has the given unique Username
    ///
    /// # Arguments
    /// * `username` The Username of the user to retrieve
    ///
    /// # Returns
    /// The user, or `None` if it wasn't found
    fn get_user_by_username(&self, username: &Username) -> Option<UserEntity>;
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
    fn get_user_by_username(&self, username: &Username) -> Option<UserEntity> {
        self.repository.get_user_by_username(username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::repository::MockUserRepository;
    use crate::{Password, UserData, UserEntity, UserID};
    use mockall::*;
    use spectral::prelude::*;

    #[test]
    fn test_get_unknown_user_by_id() {
        let user_id: UserID = Default::default();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_user_by_id()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(|_| None);

        let service = new_user_service(repository);

        let result = service.get_user_by_id(&user_id);
        assert_that(&result).is_none();
    }

    #[test]
    fn test_get_known_user_by_id() {
        let user = UserEntity {
            identity: Default::default(),
            data: UserData {
                username: "testuser".parse().unwrap(),
                email: "test@example.com".parse().unwrap(),
                display_name: "Test User".parse().unwrap(),
                password: Password::from_hash("abc"),
            },
        };

        let mut repository = MockUserRepository::new();
        let returned_user = user.clone();
        repository
            .expect_get_user_by_id()
            .with(predicate::eq(user.identity.id.clone()))
            .times(1)
            .returning(move |_| Some(returned_user.clone()));

        let service = new_user_service(repository);

        let result = service.get_user_by_id(&user.identity.id);
        assert_that(&result).is_some().is_equal_to(user);
    }

    #[test]
    fn test_get_unknown_user_by_username() {
        let username: Username = "testuser".parse().unwrap();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_user_by_username()
            .with(predicate::eq(username.clone()))
            .times(1)
            .returning(|_| None);

        let service = new_user_service(repository);

        let result = service.get_user_by_username(&username);
        assert_that(&result).is_none();
    }

    #[test]
    fn test_get_known_user_by_username() {
        let user = UserEntity {
            identity: Default::default(),
            data: UserData {
                username: "testuser".parse().unwrap(),
                email: "test@example.com".parse().unwrap(),
                display_name: "Test User".parse().unwrap(),
                password: Password::from_hash("abc"),
            },
        };

        let mut repository = MockUserRepository::new();
        let returned_user = user.clone();
        repository
            .expect_get_user_by_username()
            .with(predicate::eq(user.data.username.clone()))
            .times(1)
            .returning(move |_| Some(returned_user.clone()));

        let service = new_user_service(repository);

        let result = service.get_user_by_username(&user.data.username);
        assert_that(&result).is_some().is_equal_to(user);
    }
}
