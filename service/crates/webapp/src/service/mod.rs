use rocket::{local::Client, Rocket};
use rocket_contrib::serve::StaticFiles;
use tracing::{debug, info};

mod database;
mod templates;
mod users;

/// The actual service that we are working with
pub struct Service {
    rocket: Rocket,
}

/// Error that indicates that the creation of the service failed
#[derive(Debug, PartialEq)]
pub struct ServiceCreationError {
    message: String,
}

impl std::fmt::Display for ServiceCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ServiceCreationError {}

impl Service {
    /// Construct the service to work with
    ///
    /// # Arguments
    /// # `database_url` The connection URL for the database
    /// # `port` The port to listen on
    ///
    /// # Returns
    /// The constructed service
    pub fn new<S>(
        database_url: S,
        port: Option<u16>,
        base: S,
    ) -> Result<Service, ServiceCreationError>
    where
        S: Into<String>,
    {
        let real_base = base.into();
        let static_files = format!("{}/static", real_base);
        let message_files = format!("{}/messages/**/*.ftl", real_base);
        let template_files = format!("{}/templates/**/*.tera", real_base);
        let migration_files = format!("{}/migrations/**/*.sql", real_base);

        info!("Building universe...");
        debug!("Running in {:?}", std::env::current_dir());
        debug!("Static files: {:?}", &static_files);
        debug!("Message files: {:?}", &message_files);
        debug!("Template files: {:?}", &template_files);
        debug!("Migration files: {:?}", &migration_files);

        let database = database::new(database_url.into(), migration_files)?;

        let mut config = rocket::Config::active().unwrap();
        if let Some(port_number) = port {
            config.port = port_number;
        }

        let rocket = rocket::custom(config)
            .manage(users::new(database)?)
            .manage(templates::new(message_files, template_files)?)
            .attach(crate::server::request_id::RequestIdFairing {})
            .mount("/public", StaticFiles::from(static_files))
            .mount("/", crate::server::webapp::routes())
            .mount("/api", crate::server::rest::routes());

        Ok(Service { rocket })
    }

    /// Actually launch the server
    pub fn launch(self) {
        self.rocket.launch();
    }

    pub fn client(self) -> Client {
        Client::new(self.rocket).unwrap()
    }
}
