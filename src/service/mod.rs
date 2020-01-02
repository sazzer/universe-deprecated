use log::info;
use rocket::Rocket;

mod database;
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
    ///
    /// # Returns
    /// The constructed service
    pub fn new<S>(database_url: S) -> Result<Service, ServiceCreationError>
    where
        S: Into<String>,
    {
        info!("Building universe...");

        let database = database::new(database_url.into())?;

        let rocket = rocket::ignite()
            .manage(users::new(database)?)
            .mount("/", crate::webapp::routes())
            .mount("/api", crate::rest::routes());

        Ok(Service { rocket })
    }

    /// Actually launch the server
    pub fn launch(self) {
        self.rocket.launch();
    }
}
