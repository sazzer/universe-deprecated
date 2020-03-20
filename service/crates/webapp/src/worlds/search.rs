use super::model::World;
use crate::{page::SearchPage, problem::Problem, request_id::RequestId};
use rocket::{get, State};
use std::str::FromStr;
use tracing::debug;
use universe_entity::{parse_sorts, Pagination, SortField};
use universe_users::UserID;
use universe_worlds::{WorldFilters, WorldService, WorldSorts};

#[get("/worlds?<owner>&<keyword>&<offset>&<limit>&<sort>")]
#[tracing::instrument(skip(world_service))]
pub fn search_worlds(
  _request_id: RequestId,
  world_service: State<Box<dyn WorldService>>,
  owner: Option<String>,
  keyword: Option<String>,
  offset: Option<u32>,
  limit: Option<u32>,
  sort: Option<String>,
) -> Result<SearchPage<World>, Problem> {
  debug!("Searching worlds");

  let pagination = Pagination {
    offset: offset.unwrap_or(0),
    limit: limit.unwrap_or(10),
  };

  let sort_param = sort
    .map(|a| a.trim().to_owned())
    .filter(|b| !b.is_empty())
    .unwrap_or_else(|| "relevance".to_owned());
  let sorts: Vec<SortField<WorldSorts>> =
    parse_sorts(sort_param).map_err(|e| crate::problem::invalid_sort_fields(e))?;
  debug!("Parsed sorts: {:?}", sorts);

  let filters = WorldFilters {
    keyword,
    owner: match owner {
      None => None,
      // Technically this is a bit iffy. If we have an invalid Owner ID then we replace it with a random valid one
      // This *should* be OK because they are UUIDs and the chance of a random collision is about 1 in 10^18.4.
      // TODO: Fix this so that an invalid User ID actually means we don't even bother doing the search
      Some(owner) => Some(UserID::from_str(&owner).unwrap_or_default()),
    },
  };
  debug!("Parsed filters: {:?}", filters);

  let results = world_service.search_worlds(filters, sorts, pagination);
  debug!("Matching worlds: {:?}", results);
  Ok(results.into())
}
