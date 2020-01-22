/// Definition of something that can check it's own health
pub trait Healthcheck: Send + Sync {
    /// Check the health of the database connection.
    ///
    /// # Returns
    /// The result of the healthcheck
    fn check_health(&self) -> Result<String, String>;
}
