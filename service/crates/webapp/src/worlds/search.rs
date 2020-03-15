use crate::problem::Problem;
use crate::request_id::RequestId;
use rocket::get;
use universe_entity::Pagination;

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
  tracing::info!("Searching worlds");

  let pagination = Pagination {
    offset: offset.unwrap_or(0),
    limit: limit.unwrap_or(10),
  };
  Ok("Hello".to_owned())
}
