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

#[cfg(test)]
impl From<universe_testdata::World> for WorldEntity {
  fn from(world: universe_testdata::World) -> WorldEntity {
    WorldEntity {
      identity: Identity {
        id: WorldID::from_uuid(world.world_id),
        version: world.version,
        created: world.created,
        updated: world.updated,
      },
      data: WorldData {
        owner: UserID::from_uuid(world.owner_id),
        name: world.name.clone(),
        slug: world.slug.parse().unwrap(),
        description: world.description.clone(),
      },
    }
  }
}
