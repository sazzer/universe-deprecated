use rocket::{local::Client, Rocket};
use rocket_contrib::serve::StaticFiles;
use tracing::info;

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
    pub fn new<S>(database_url: S, port: Option<u16>) -> Result<Service, ServiceCreationError>
    where
        S: Into<String>,
    {
        info!("Building universe...");

        let database = database::new(database_url.into())?;

        let mut config = rocket::Config::active().unwrap();
        if let Some(port_number) = port {
            config.port = port_number;
        }

        let rocket = rocket::custom(config)
            .manage(users::new(database)?)
            .manage(templates::new()?)
            .attach(crate::server::request_id::RequestIdFairing {})
            .mount("/public", StaticFiles::from("./static"))
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