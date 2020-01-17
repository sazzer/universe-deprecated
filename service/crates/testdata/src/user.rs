use chrono::{DateTime, Timelike, Utc};
use universe_test_database_wrapper::TestData;
use uuid::Uuid;

/// User representation for test purposes
#[derive(Debug, Clone)]
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

    fn binds(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)> {
        vec![
            &self.user_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.username,
            &self.email,
            &self.display_name,
            &self.password,
        ]
    }
}
