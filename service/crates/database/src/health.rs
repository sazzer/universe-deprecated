use crate::postgres::Database;
use universe_health::Healthcheck;

impl Healthcheck for Database {
    /// Check the health of the database connection.
    /// This works by checking out a connection and performing a query that is known to work - SELECT 1
    ///
    /// # Returns
    /// The result of the healthcheck
    fn check_health(&self) -> Result<String, String> {
        let mut client = self
            .client()
            .ok_or_else(|| "Failed to get connection".to_owned())?;

        let _ = client
            .query("SELECT 1", &[])
            .map_err(|e| format!("Error querying database: {}", e))?;

        Ok("Ok".to_owned())
    }
}
