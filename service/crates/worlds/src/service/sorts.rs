use strum_macros::{EnumString, EnumVariantNames};

/// Fields that we can sort by when searching worlds
#[derive(Debug, PartialEq, EnumVariantNames, EnumString)]
pub enum WorldSorts {
  Name,
  Owner,
  Created,
  Relevance,
  Id,
}
