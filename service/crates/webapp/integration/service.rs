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
        let _ = tracing_subscriber::fmt::try_init();

        let service_base = std::fs::canonicalize("../..").unwrap();
        let migrations_path = service_base.join("migrations");
        let migrations_glob = format!("{}/**/*.sql", migrations_path.to_str().unwrap());

        let database = TestDatabaseWrapper::new();

        let webapp = Service::new(
            &database.url(),
            None,
            "accessTokenSecretKey",
            &migrations_glob,
        );
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

    pub fn get<'c, 'u: 'c, U: Into<std::borrow::Cow<'u, str>>>(
        &'c self,
        uri: U,
    ) -> rocket::local::LocalRequest<'c> {
        self.client().get(uri).header(rocket::http::Header::new(
            "X-Request-Client-Name",
            "IntegrationTest",
        ))
    }

    pub fn post<'c, 'u: 'c, U: Into<std::borrow::Cow<'u, str>>>(
        &'c self,
        uri: U,
    ) -> rocket::local::LocalRequest<'c> {
        self.client().post(uri).header(rocket::http::Header::new(
            "X-Request-Client-Name",
            "IntegrationTest",
        ))
    }

    pub fn patch<'c, 'u: 'c, U: Into<std::borrow::Cow<'u, str>>>(
        &'c self,
        uri: U,
    ) -> rocket::local::LocalRequest<'c> {
        self.client().patch(uri).header(rocket::http::Header::new(
            "X-Request-Client-Name",
            "IntegrationTest",
        ))
    }

    /// Get the database the service is using
    pub fn database(&self) -> &TestDatabaseWrapper<'d> {
        &self.database
    }
}
