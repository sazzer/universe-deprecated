use crate::{
    postgres::PostgresUserRepository, service::implementation::UserServiceImpl, UserService,
};
use std::sync::Arc;
use universe_database::Database;

/// Construct the new User Service to work with
///
/// # Arguments
/// # `database` The database connection to use
///
/// # Returns
/// The user service
pub fn new(database: Arc<dyn Database>) -> Arc<dyn UserService> {
    let repository = Arc::new(PostgresUserRepository::new(database));
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(repository));

    user_service
}
