use super::{WorldFilters, WorldSorts};
use crate::model::*;
#[cfg(test)]
use mockall::automock;
use universe_entity::{Page, Pagination, SortField};

/// Repository that describes how to access world data
#[cfg_attr(test, automock)]
pub trait WorldRepository {
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
    filters: WorldFilters,
    sorts: Vec<SortField<WorldSorts>>,
    pagination: Pagination,
  ) -> Page<WorldEntity>;
}
