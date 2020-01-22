use serde::Serialize;

/// Representation of a User to return over the API
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub display_name: String,
}
