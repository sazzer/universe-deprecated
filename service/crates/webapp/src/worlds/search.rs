use crate::problem::Problem;
use crate::request_id::RequestId;
use rocket::get;
use tracing::info;
use universe_entity::{parse_sorts, Pagination, SortDirection, SortField};
use universe_worlds::WorldSorts;

#[get("/worlds?<owner>&<keyword>&<offset>&<limit>&<sort>")]
#[tracing::instrument(skip())]
pub fn search_worlds(
  _request_id: RequestId,
  owner: Option<String>,
  keyword: Option<String>,
  offset: Option<u32>,
  limit: Option<u32>,
  sort: Option<String>,
) -> Result<String, Problem> {
  info!("Searching worlds");

  let _pagination = Pagination {
    offset: offset.unwrap_or(0),
    limit: limit.unwrap_or(10),
  };

  let sort_param = sort
    .map(|a| a.trim().to_owned())
    .filter(|b| !b.is_empty())
    .unwrap_or_else(|| "relevance".to_owned());
  let mut _sorts: Vec<SortField<WorldSorts>> =
    parse_sorts(sort_param).map_err(|e| crate::problem::invalid_sort_fields(e))?;
  _sorts.push(SortField {
    field: WorldSorts::Id,
    direction: SortDirection::Natural,
  });
  info!("Parsed sorts: {:?}", _sorts);

  Ok("Hello".to_owned())
}
