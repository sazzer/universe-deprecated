use crate::{Slug, WorldID};
use universe_entity::Identity;
use universe_users::UserID;

/// Struct to represent the data about a single World
#[derive(Debug, PartialEq, Clone)]
pub struct WorldData {
  pub owner: UserID,
  pub name: String,
  pub slug: Slug,
  pub description: String,
}

/// Type to represent the entity that is a persisted world record
#[derive(Debug, PartialEq, Clone)]
pub struct WorldEntity {
  pub identity: Identity<WorldID>,
  pub data: WorldData,
}
