use super::{migrate::MigratableDatabase, postgres::PostgresDatabase};
use lazy_static::lazy_static;
use log::info;
use std::sync::Arc;
use testcontainers::*;

lazy_static! {
    static ref DOCKER: clients::Cli = clients::Cli::default();
}

/// Wrapper around the database for test purposes
pub struct TestDatabase<'a> {
    pub database: Arc<PostgresDatabase>,
    container: testcontainers::core::Container<'a, clients::Cli, images::postgres::Postgres>,
}

impl<'a> TestDatabase<'a> {
    pub fn new() -> Self {
        let node = DOCKER.run(images::postgres::Postgres::default());

        let host_port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres:postgres@localhost:{}", host_port);
        info!("Running postgres on {}", url);

        let postgres_database = PostgresDatabase::new(url).unwrap();
        postgres_database.migrate("migrations").unwrap();

        TestDatabase {
            database: Arc::new(postgres_database),
            container: node,
        }
    }
}
