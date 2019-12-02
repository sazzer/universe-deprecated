use super::{service::UserRepository, UserEntity, Username};
use log::debug;

/// User Repository that works in terms of a PostgreSQL database connection
pub struct PostgresUserRepository {}

impl UserRepository for PostgresUserRepository {
    fn get_by_username(&self, username: Username) -> Option<UserEntity> {
        debug!("Looking up username {:?}", username);

        None
    }
}
