use super::{WorldFilters, WorldSorts};
use crate::{model::*, service::repository::*};
use universe_database::Database;
use universe_entity::{Page, Pagination, SortField};

impl WorldRepository for Database {
  /// Perform a search for all the worlds that match the given filters, sorted in the requested order.
  ///
  /// # Arguments
  /// * `filters` The filters to apply when searching for worlds
  /// * `sorts` The sorts to apply when sorting the worlds
  /// * `pagination` The pagination details for which set of data to return
  ///
  /// # Returns
  /// A page of worlds
  fn search_worlds(
    &self,
    _filters: WorldFilters,
    _sorts: Vec<SortField<WorldSorts>>,
    pagination: Pagination,
  ) -> Page<WorldEntity> {
    Page {
      entries: vec![],
      total: 0,
      offset: pagination.offset,
    }
  }
}
