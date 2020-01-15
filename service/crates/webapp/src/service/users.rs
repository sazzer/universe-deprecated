use super::ServiceCreationError;
use crate::database::Database;
use crate::users::{
    implementation::UserServiceImpl, postgres::PostgresUserRepository, UserService,
};
use std::sync::Arc;

/// Construct the new User Service to work with
///
/// # Arguments
/// # `database` The database connection to use
///
/// # Returns
/// The user service
pub fn new(database: Arc<dyn Database>) -> Result<Arc<dyn UserService>, ServiceCreationError> {
    let repository = Arc::new(PostgresUserRepository::new(database));
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(repository));

    Ok(user_service)
}
