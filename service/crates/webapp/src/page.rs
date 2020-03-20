use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchPage<T>
where
  T: Serialize,
{
  pub total: u32,
  pub entries: Vec<T>,
}
