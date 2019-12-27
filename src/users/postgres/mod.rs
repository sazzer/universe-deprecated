use crate::database::Database;
use crate::users::repository::UserRepository;
use std::sync::Arc;

pub struct PostgresUserRepository {
    #[allow(dead_code)]
    database: Arc<dyn Database>,
}

impl PostgresUserRepository {
    pub fn new(database: Arc<dyn Database>) -> Self {
        Self { database }
    }
}
impl UserRepository for PostgresUserRepository {}
