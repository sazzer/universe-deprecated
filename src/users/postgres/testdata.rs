use crate::database::test::TestData;
use crate::users::{Password, UserID, Username};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub user_id: UserID,
    pub version: Uuid,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub username: Username,
    pub email: String,
    pub display_name: String,
    pub password: Password,
}

impl Default for User {
    fn default() -> Self {
        Self {
            user_id: UserID::from_uuid(Uuid::new_v4()),
            version: Uuid::new_v4(),
            created: Utc::now(),
            updated: Utc::now(),
            username: "testuser".parse().unwrap(),
            email: "test@example.com".to_owned(),
            display_name: "Test User".to_owned(),
            password: Password::from_plaintext("password").unwrap(),
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
