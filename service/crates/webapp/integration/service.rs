use rocket::local::Client;
use universe_test_database_wrapper::TestDatabaseWrapper;
use universe_webapp::Service;

/// Wrapper around the service being tested
pub struct ServiceWrapper<'d> {
    database: TestDatabaseWrapper<'d>,
    webapp: Client,
}

impl<'d> Default for ServiceWrapper<'d> {
    fn default() -> Self {
        let _ = tracing_subscriber::fmt::Builder::default()
            .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
            .try_init();

        let service_base = std::fs::canonicalize("../..").unwrap();
        let migrations_path = service_base.join("migrations");
        let migrations_glob = format!("{}/**/*.sql", migrations_path.to_str().unwrap());

        let database = TestDatabaseWrapper::new();

        let webapp = Service::new(&database.url(), None, &migrations_glob);
        Self {
            database,
            webapp: webapp.client(),
        }
    }
}

impl<'d> ServiceWrapper<'d> {
    /// Get the HTTP Client used for interacting with the service
    pub fn client(&self) -> &Client {
        &self.webapp
    }

    /// Get the database the service is using
    pub fn database(&self) -> &TestDatabaseWrapper<'d> {
        &self.database
    }
}
