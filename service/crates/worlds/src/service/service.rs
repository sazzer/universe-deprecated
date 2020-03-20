use super::{WorldFilters, WorldSorts};
use crate::model::*;
use universe_entity::{Page, Pagination, SortField};

/// The World Service to allow interactoins with worldentities
pub trait WorldService: Send + Sync {
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
