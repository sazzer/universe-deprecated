use super::repository::UserRepository;
use super::UserService;
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

impl UserService for UserServiceImpl {}
