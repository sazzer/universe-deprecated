use postgres_types::ToSql;
use std::boxed::Box;
use std::ops::Deref;
use universe_database::Database;

/// Trait that all test data types can implement to allow them to be inserted into a database
pub trait TestData {
    /// Generate the SQL needed to insert the data into the database
    fn sql(&self) -> String;
    /// Generate the binds needed to insert the data into the database
    fn binds(&self) -> Vec<Box<(dyn ToSql + Sync)>>;
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
        let sql = d.sql();
        let binds = d.binds();
        let real_binds: Vec<&(dyn ToSql + Sync)> = binds.iter().map(|b| b.deref()).collect();

        transaction.query(sql.as_str(), &real_binds).unwrap();
    }

    transaction.commit().unwrap();
}
