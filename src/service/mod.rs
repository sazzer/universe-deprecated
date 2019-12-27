mod database;
mod users;

/// The actual service that we are working with
pub struct Service {}

/// Error that indicates that the creation of the service failed
#[derive(Debug, PartialEq)]
pub struct ServiceCreationError {}

impl Service {
    /// Construct the service to work with
    ///
    /// # Arguments
    /// # `database_url` The connection URL for the database
    ///
    /// # Returns
    /// The constructed service
    pub fn new(database_url: String) -> Result<Service, ServiceCreationError> {
        let database = database::new(database_url)?;
        let _user_service = users::new(database)?;

        Ok(Service {})
    }
}
