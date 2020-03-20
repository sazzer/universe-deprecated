use super::{repository::*, service::*, WorldFilters, WorldSorts};
use crate::model::*;
use universe_entity::{Page, Pagination, SortField};

/// The World Service to allow interactoins with world entities
pub struct WorldServiceImpl<Repo> {
  repository: Repo,
}

/// Create a new World Service
///
/// # Returns
/// The World Service
pub fn new_world_service<Repo: WorldRepository + Send + Sync>(
  repository: Repo,
) -> impl WorldService {
  WorldServiceImpl { repository }
}

impl<Repo: WorldRepository + Send + Sync> WorldService for WorldServiceImpl<Repo> {
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
  ) -> Page<WorldEntity> {
    self.repository.search_worlds(filters, sorts, pagination)
  }
}
