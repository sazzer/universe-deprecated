use crate::headers::*;
use crate::page::SearchPage;
use chrono::{DateTime, Utc};
use rocket::{
  http::{
    hyper::header::{CacheControl, CacheDirective, ETag, EntityTag, HttpDate, LastModified},
    Status,
  },
  response::{Responder, Response},
  Request,
};
use rocket_contrib::json::Json;
use serde::Serialize;
use universe_entity::Page;
use universe_worlds::*;
use uuid::Uuid;

/// Representation of a World to return over the API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
  pub id: WorldID,
  #[serde(skip_serializing)]
  version: Uuid,
  #[serde(skip_serializing)]
  updated: DateTime<Utc>,
}

impl<'a> Responder<'a> for World {
  /// Generate a Rocket response for the User
  fn respond_to(self, req: &Request) -> Result<Response<'a>, Status> {
    Response::build()
      .merge(Json(&self).respond_to(req)?)
      .header(Link::from_href(format!("/users/{}", self.id)).with_rel("self"))
      .header(AcceptPatch("application/merge-patch+json"))
      .header(ETag(EntityTag::new(false, self.version.to_string())))
      .header(LastModified(HttpDate(time::at_utc(time::Timespec::new(
        self.updated.timestamp(),
        0,
      )))))
      .header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(3600),
      ]))
      .ok()
  }
}

impl From<&WorldEntity> for World {
  fn from(world: &WorldEntity) -> Self {
    World {
      id: world.identity.id.clone(),
      version: world.identity.version,
      updated: world.identity.updated,
    }
  }
}
impl From<WorldEntity> for World {
  fn from(world: WorldEntity) -> Self {
    World::from(&world)
  }
}

impl<'a> Responder<'a> for SearchPage<World> {
  /// Generate a Rocket response for the User
  fn respond_to(self, req: &Request) -> Result<Response<'a>, Status> {
    Response::build()
      .merge(Json(&self).respond_to(req)?)
      .header(CacheControl(vec![
        CacheDirective::Public,
        CacheDirective::MaxAge(3600),
      ]))
      .ok()
  }
}

impl From<Page<WorldEntity>> for SearchPage<World> {
  fn from(worlds: Page<WorldEntity>) -> Self {
    SearchPage {
      entries: worlds.entries.iter().map(|world| world.into()).collect(),
      total: worlds.total,
    }
  }
}
