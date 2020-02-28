use super::repository::*;
use super::service::*;
use crate::model::*;
use tracing::warn;

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

    fn register_user(&self, user: UserData) -> Result<UserEntity, RegisterUserError> {
        let created = self.repository.create_user(user)?;
        Ok(created)
    }

    fn update_user(
        &self,
        user_id: &UserID,
        updater: &mut dyn FnMut(UserData) -> UserData,
    ) -> Result<UserEntity, UpdateUserError> {
        let user = self
            .get_user_by_id(&user_id)
            .ok_or(UpdateUserError::UnknownUser)?;

        let updated = updater(user.data);

        let saved = self.repository.update_user(UserEntity {
            identity: user.identity,
            data: updated,
        })?;
        Ok(saved)
    }
}

impl From<PersistUserError> for RegisterUserError {
    fn from(e: PersistUserError) -> Self {
        warn!("Error creating user: {}", e);
        match e {
            PersistUserError::DuplicateUsername => {
                RegisterUserError::ValidationError(vec![UserValidationError::DuplicateUsername])
            }
            PersistUserError::DuplicateEmail => {
                RegisterUserError::ValidationError(vec![UserValidationError::DuplicateEmail])
            }
            _ => RegisterUserError::UnknownError,
        }
    }
}

impl From<PersistUserError> for UpdateUserError {
    fn from(e: PersistUserError) -> Self {
        warn!("Error creating user: {}", e);
        match e {
            PersistUserError::DuplicateUsername => {
                UpdateUserError::ValidationError(vec![UserValidationError::DuplicateUsername])
            }
            PersistUserError::DuplicateEmail => {
                UpdateUserError::ValidationError(vec![UserValidationError::DuplicateEmail])
            }
            PersistUserError::UserNotFound => UpdateUserError::UnknownUser,
            PersistUserError::OptimisticLockFailure => UpdateUserError::OptimisticLockFailure,
            _ => UpdateUserError::UnknownError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::repository::MockUserRepository;
    use crate::{Password, UserData, UserEntity, UserID};
    use mockall::*;
    use spectral::prelude::*;
    // use test_env_log::test;

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

    #[test]
    fn test_register_user_success() {
        let user_data = UserData {
            username: "testuser".parse().unwrap(),
            email: "test@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_hash("abc"),
        };

        let new_user = UserEntity {
            identity: Default::default(),
            data: user_data.clone(),
        };

        let mut repository = MockUserRepository::new();
        repository
            .expect_create_user()
            .with(predicate::eq(user_data.clone()))
            .times(1)
            .returning(move |_user| Ok(new_user.clone()));

        let service = new_user_service(repository);

        let created_user_data = user_data.clone();
        let result = service.register_user(created_user_data).unwrap();
        assert_that(&result.data).is_equal_to(user_data);
    }

    #[test]
    fn test_register_user_unexpected_error() {
        let user_data = UserData {
            username: "testuser".parse().unwrap(),
            email: "test@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_hash("abc"),
        };

        let mut repository = MockUserRepository::new();
        repository
            .expect_create_user()
            .with(predicate::eq(user_data.clone()))
            .times(1)
            .returning(|_| Err(PersistUserError::UnknownError));

        let service = new_user_service(repository);

        let result = service.register_user(user_data.clone());
        assert_that(&result)
            .is_err()
            .is_equal_to(RegisterUserError::UnknownError);
    }

    #[test]
    fn test_register_user_duplicate_username() {
        let user_data = UserData {
            username: "testuser".parse().unwrap(),
            email: "test@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_hash("abc"),
        };

        let mut repository = MockUserRepository::new();
        repository
            .expect_create_user()
            .with(predicate::eq(user_data.clone()))
            .times(1)
            .returning(|_| Err(PersistUserError::DuplicateUsername));

        let service = new_user_service(repository);

        let result = service.register_user(user_data.clone());
        assert_that(&result)
            .is_err()
            .is_equal_to(RegisterUserError::ValidationError(vec![
                UserValidationError::DuplicateUsername,
            ]));
    }

    #[test]
    fn test_register_user_duplicate_email() {
        let user_data = UserData {
            username: "testuser".parse().unwrap(),
            email: "test@example.com".parse().unwrap(),
            display_name: "Test User".parse().unwrap(),
            password: Password::from_hash("abc"),
        };

        let mut repository = MockUserRepository::new();
        repository
            .expect_create_user()
            .with(predicate::eq(user_data.clone()))
            .times(1)
            .returning(|_| Err(PersistUserError::DuplicateEmail));

        let service = new_user_service(repository);

        let result = service.register_user(user_data.clone());
        assert_that(&result)
            .is_err()
            .is_equal_to(RegisterUserError::ValidationError(vec![
                UserValidationError::DuplicateEmail,
            ]));
    }

    #[test]
    fn test_update_unknown_user() {
        let user_id: UserID = Default::default();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_user_by_id()
            .with(predicate::eq(user_id.clone()))
            .times(1)
            .returning(|_| None);

        let service = new_user_service(repository);
        let result = service.update_user(&user_id, &mut |user| user);

        assert_that(&result)
            .is_err()
            .is_equal_to(UpdateUserError::UnknownUser);
    }

    #[test]
    fn test_update_no_changes() {
        // The user as it currently exists
        let user = UserEntity {
            identity: Default::default(),
            data: UserData {
                username: "testuser".parse().unwrap(),
                email: "test@example.com".parse().unwrap(),
                display_name: "Test User".parse().unwrap(),
                password: Password::from_hash("abc"),
            },
        };

        // The user that we expect to see when calling to save in the repository. This has the data mutated.
        let expected_user = UserEntity {
            identity: user.identity.clone(),
            data: user.data.clone(),
        };

        // The user as returned after saving in the repostory. This has a different Identity.
        let updated_user = UserEntity {
            identity: Default::default(),
            data: UserData {
                ..expected_user.data.clone()
            },
        };

        // The mock repository
        let mut repository = MockUserRepository::new();

        let returned_user = user.clone();
        repository
            .expect_get_user_by_id()
            .with(predicate::eq(user.identity.id.clone()))
            .times(1)
            .returning(move |_| Some(returned_user.clone()));

        let returned_updated_user = updated_user.clone();
        repository
            .expect_update_user()
            .with(predicate::eq(expected_user))
            .times(1)
            .returning(move |_| Ok(returned_updated_user.clone()));

        // The service being tested
        let service = new_user_service(repository);
        let result = service.update_user(&user.identity.id, &mut |user| user);

        assert_that(&result)
            .is_ok()
            .is_equal_to(updated_user.clone());
    }

    #[test]
    fn test_update_with_changes() {
        // The user as it currently exists
        let user = UserEntity {
            identity: Default::default(),
            data: UserData {
                username: "testuser".parse().unwrap(),
                email: "test@example.com".parse().unwrap(),
                display_name: "Test User".parse().unwrap(),
                password: Password::from_hash("abc"),
            },
        };

        // The user that we expect to see when calling to save in the repository. This has the data mutated.
        let expected_user = UserEntity {
            identity: user.identity.clone(),
            data: UserData {
                email: "new@example.com".parse().unwrap(),
                display_name: "New User".parse().unwrap(),
                ..user.data.clone()
            },
        };

        // The user as returned after saving in the repostory. This has a different Identity.
        let updated_user = UserEntity {
            identity: Default::default(),
            data: UserData {
                ..expected_user.data.clone()
            },
        };

        // The mock repository
        let mut repository = MockUserRepository::new();

        let returned_user = user.clone();
        repository
            .expect_get_user_by_id()
            .with(predicate::eq(user.identity.id.clone()))
            .times(1)
            .returning(move |_| Some(returned_user.clone()));

        let returned_updated_user = updated_user.clone();
        repository
            .expect_update_user()
            .with(predicate::eq(expected_user))
            .times(1)
            .returning(move |_| Ok(returned_updated_user.clone()));

        // The service being tested
        let service = new_user_service(repository);
        let result = service.update_user(&user.identity.id, &mut |mut user| {
            user.email = "new@example.com".parse().unwrap();
            user.display_name = "New User".parse().unwrap();
            user
        });

        assert_that(&result)
            .is_ok()
            .is_equal_to(updated_user.clone());
    }

    #[test]
    fn test_update_validation_error() {
        // The user as it currently exists
        let user = UserEntity {
            identity: Default::default(),
            data: UserData {
                username: "testuser".parse().unwrap(),
                email: "test@example.com".parse().unwrap(),
                display_name: "Test User".parse().unwrap(),
                password: Password::from_hash("abc"),
            },
        };

        // The user that we expect to see when calling to save in the repository. This has the data mutated.
        let expected_user = UserEntity {
            identity: user.identity.clone(),
            data: UserData {
                email: "new@example.com".parse().unwrap(),
                display_name: "New User".parse().unwrap(),
                ..user.data.clone()
            },
        };

        // The mock repository
        let mut repository = MockUserRepository::new();

        let returned_user = user.clone();
        repository
            .expect_get_user_by_id()
            .with(predicate::eq(user.identity.id.clone()))
            .times(1)
            .returning(move |_| Some(returned_user.clone()));

        repository
            .expect_update_user()
            .with(predicate::eq(expected_user))
            .times(1)
            .returning(move |_| Err(PersistUserError::DuplicateEmail));

        // The service being tested
        let service = new_user_service(repository);
        let result = service.update_user(&user.identity.id, &mut |mut user| {
            user.email = "new@example.com".parse().unwrap();
            user.display_name = "New User".parse().unwrap();
            user
        });

        assert_that(&result)
            .is_err()
            .is_equal_to(UpdateUserError::ValidationError(vec![
                UserValidationError::DuplicateEmail,
            ]));
    }
}
