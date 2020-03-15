/// Struct representing the pagination controls when listing data from a service
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pagination {
  pub offset: u32,
  pub limit: u32,
}
