use crate::testdata::TestData;
use chrono::{DateTime, Timelike, Utc};
use postgres_types::ToSql;
use std::boxed::Box;
use uuid::Uuid;

/// Test Data for a World record
#[derive(Debug, PartialEq, Clone)]
pub struct World {
  pub world_id: Uuid,
  pub version: Uuid,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
  pub owner_id: Uuid,
  pub name: String,
  pub slug: String,
  pub description: String,
}

impl Default for World {
  /// Generate a default set of values for the test World structure
  fn default() -> Self {
    Self {
      world_id: Uuid::new_v4(),
      version: Uuid::new_v4(),
      created: Utc::now().with_nanosecond(0).unwrap(),
      updated: Utc::now().with_nanosecond(0).unwrap(),
      owner_id: Uuid::new_v4(),
      name: "Test World".to_owned(),
      slug: "test-world".to_owned(),
      description: "This is a test world".to_owned(),
    }
  }
}

impl TestData for World {
  fn sql(&self) -> String {
    "INSERT INTO worlds(world_id, version, created, updated, owner_id, name, slug, description) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)".to_owned()
  }

  fn binds(&self) -> Vec<Box<(dyn ToSql + Sync)>> {
    vec![
      Box::new(self.world_id),
      Box::new(self.version),
      Box::new(self.created),
      Box::new(self.updated),
      Box::new(self.owner_id),
      Box::new(self.name.clone()),
      Box::new(self.slug.clone()),
      Box::new(self.description.clone()),
    ]
  }
}
