use rocket::FromForm;
use serde::Serialize;

/// Form data to represent the various login forms that we might have.
///
/// This needs to cover the initial login form, containing only the username, as well as the forms
/// on the Register page and on the Login page.
#[derive(FromForm, Debug, Serialize)]
pub struct LoginForm {
    pub action: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub password2: Option<String>,
}

impl LoginForm {
    pub fn get_action(&self) -> String {
        match &self.action {
            Some(a) => a.to_string(),
            None => "".to_owned(),
        }
    }
}
