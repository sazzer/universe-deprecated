use super::{repository::UserRepository, UserRegistrationError, UserService};
use crate::{UserData, UserEntity, UserID, Username};
use std::sync::Arc;

/// Standard implementation of the User Service
pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}

impl UserService for UserServiceImpl {
    fn get_user_by_id(&self, user_id: &UserID) -> Option<UserEntity> {
        self.repository.get_user_by_id(user_id)
    }

    fn username_exists(&self, username: &Username) -> bool {
        self.repository
            .get_user_by_username(username)
            .map(|_| true)
            .unwrap_or(false)
    }

    fn register_user(&self, _user: UserData) -> Result<UserEntity, Vec<UserRegistrationError>> {
        Err(vec![UserRegistrationError::DuplicateEmailAddress])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::repository::MockUserRepository;
    use mockall::predicate::*;
    use spectral::prelude::*;
    use std::str::FromStr;
    use universe_testdata::User;
    use uuid::Uuid;

    #[test]
    fn test_get_unknown_user_by_id() {
        let mut repository = MockUserRepository::new();

        let user_id = UserID::from_uuid(Uuid::new_v4());

        repository
            .expect_get_user_by_id()
            .with(eq(user_id))
            .times(1)
            .returning(|_| None);

        let service = UserServiceImpl::new(Arc::new(repository));
        let user = service.get_user_by_id(&user_id);

        assert_that(&user).is_none();
    }

    #[test]
    fn test_get_known_user_by_id() {
        let mut repository = MockUserRepository::new();

        let testuser = User::default();
        let user_entity = UserEntity::from(testuser.clone());

        repository
            .expect_get_user_by_id()
            .with(eq(UserID::from_uuid(testuser.user_id)))
            .times(1)
            .returning(move |_| Some(user_entity.clone()));

        let service = UserServiceImpl::new(Arc::new(repository));

        let user_id: UserID = UserID::from_uuid(testuser.user_id);
        let user = service.get_user_by_id(&user_id);

        assert_that(&user)
            .is_some()
            .is_equal_to(UserEntity::from(testuser.clone()));
    }

    #[test]
    fn test_unknown_username_exists() {
        let mut repository = MockUserRepository::new();

        let username: Username = "testuser".parse().unwrap();
        repository
            .expect_get_user_by_username()
            .with(eq(username.clone()))
            .times(1)
            .returning(|_| None);

        let service = UserServiceImpl::new(Arc::new(repository));

        let result = service.username_exists(&username);

        assert_that(&result).is_equal_to(false);
    }

    #[test]
    fn test_known_username_exists() {
        let mut repository = MockUserRepository::new();

        let testuser = User::default();
        let user_entity = UserEntity::from(testuser.clone());

        repository
            .expect_get_user_by_username()
            .with(eq(Username::from_str(&testuser.username).unwrap()))
            .times(1)
            .returning(move |_| Some(user_entity.clone()));

        let service = UserServiceImpl::new(Arc::new(repository));

        let username: Username = testuser.username.clone().parse().unwrap();
        let result = service.username_exists(&username);

        assert_that(&result).is_equal_to(true);
    }
}
