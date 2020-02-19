use crate::testdata::TestData;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Timelike, Utc};
use postgres_types::ToSql;
use std::boxed::Box;
use uuid::Uuid;

/// Test Data for a User record
#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub password: String,
}

impl Default for User {
    /// Generate a default set of values for the test User structure
    fn default() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            version: Uuid::new_v4(),
            created: Utc::now().with_nanosecond(0).unwrap(),
            updated: Utc::now().with_nanosecond(0).unwrap(),
            username: "testuser".to_owned(),
            email: "test@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            password: "password".to_owned(),
        }
    }
}

impl TestData for User {
    fn sql(&self) -> String {
        "INSERT INTO users(user_id, version, created, updated, username, email, display_name, password) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)".to_owned()
    }

    fn binds(&self) -> Vec<Box<(dyn ToSql + Sync)>> {
        let hashed_password = hash(&self.password, DEFAULT_COST).unwrap();

        vec![
            Box::new(self.user_id),
            Box::new(self.version),
            Box::new(self.created),
            Box::new(self.updated),
            Box::new(self.username.clone()),
            Box::new(self.email.clone()),
            Box::new(self.display_name.clone()),
            Box::new(hashed_password),
        ]
    }
}
