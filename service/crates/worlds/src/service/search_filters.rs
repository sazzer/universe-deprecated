use universe_users::UserID;

/// Filters that can be applied when searching for worlds
#[derive(Debug, PartialEq)]
pub struct WorldFilters {
  pub owner: Option<UserID>,
  pub keyword: Option<String>,
}
