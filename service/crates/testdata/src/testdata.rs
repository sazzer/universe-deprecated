use postgres_types::ToSql;
use universe_database::Database;

/// Trait that all test data types can implement to allow them to be inserted into a database
pub trait TestData {
    /// Generate the SQL needed to insert the data into the database
    fn sql(&self) -> String;
    /// Generate the binds needed to insert the data into the database
    fn binds(&self) -> Vec<&(dyn ToSql + Sync)>;
}

/// Seed a set of data into the given database
///
/// # Arguments
/// * `database` The database to seed
/// * `data` The data to seed
pub fn seed(database: &Database, data: Vec<&dyn TestData>) {
    let mut client = database.client().unwrap();
    let mut transaction = client.transaction().unwrap();

    for d in data.iter() {
        transaction
            .query(d.sql().as_str(), &(d.binds()[..]))
            .unwrap();
    }

    transaction.commit().unwrap();
}
