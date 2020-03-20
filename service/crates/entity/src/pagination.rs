/// Struct representing the pagination controls when listing data from a service
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pagination {
  pub offset: u32,
  pub limit: u32,
}

/// Struct representing a page of entities returned from some service
#[derive(Debug)]
pub struct Page<T> {
  pub offset: u32,
  pub total: u32,
  pub entries: Vec<T>,
}
