use std::sync::Arc;
use tracing::debug;
use universe_database::Database;

pub struct Service {
    #[allow(dead_code)]
    database: Arc<dyn Database>,
}

impl Service {
    pub fn new(database_url: &str, migration_files: &str) -> Self {
        debug!("Building Universe...");

        let database = universe_database::builder::new(database_url, migration_files).unwrap();

        Service { database }
    }
}
