use super::{Migrations, MigrationsError};
use log::{debug, error};
use postgres::transaction;
use r2d2_postgres::PostgresConnectionManager;
use std::fs;

impl From<postgres::Error> for MigrationsError<'_> {
    fn from(err: postgres::Error) -> Self {
        error!("Postgres Error: {:?}", err);
        MigrationsError::DatabaseError(err.to_string().into())
    }
}

impl Migrations<PostgresConnectionManager> {
    /// Actually execute the migrations.
    ///
    /// This is idempotent and can be executed as many times as needed and the results should always
    /// be the same on the same input.
    ///
    /// # Returns
    /// The number of migrations that were actually applied. This could be 0 if there were no outstanding
    /// migrations to apply.
    pub fn migrate<'a>(&self) -> Result<u32, MigrationsError<'a>> {
        // Execute the entire chunk of work within a single transaction
        let mut transaction_config = transaction::Config::new();
        transaction_config.isolation_level(transaction::IsolationLevel::Serializable);
        transaction_config.read_only(false);
        let transaction = self.conn.transaction_with(&transaction_config)?;

        // Create our __migrations table to record the migrations we are running
        transaction.execute(
            "CREATE TABLE IF NOT EXISTS __migrations(
                    migration_file TEXT PRIMARY KEY,
                    sequence SERIAL NOT NULL,
                    executed TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    executed_from TEXT NOT NULL DEFAULT inet_client_addr()
                )",
            &[],
        )?;

        // Lock the table, so that if two migrations try to run at the same time then they will not
        // interfere with each other
        transaction.execute("LOCK TABLE __migrations IN EXCLUSIVE MODE", &[])?;

        // Determine the set of all migrations that have been run to date
        let applied_migrations: Vec<String> = transaction
            .query("SELECT migration_file FROM __migrations", &[])?
            .iter()
            .map(|row| row.get::<&str, String>("migration_file"))
            .collect::<Vec<String>>();

        // And finally try to actually apply the migrations that haven't yet been run
        let mut applied = 0;

        for entry in self.files.iter().filter_map(|entry| entry.to_str()) {
            if applied_migrations.contains(&entry.to_owned()) {
                debug!("Already processed file: {:?}", entry);
            } else {
                debug!("Processing file: {:?}", entry);
                let source = fs::read_to_string(&entry)?;

                transaction.execute(&source, &[])?;

                transaction.execute(
                    "INSERT INTO __migrations(migration_file) VALUES ($1)",
                    &[&entry],
                )?;
                applied += 1;
            }
        }
        debug!("Applied {} out of {} migrations", applied, self.files.len());

        // When all is done, commit the transaction
        transaction.commit()?;

        Ok(applied)
    }
}
