use super::form::LoginForm;
use crate::users::{UserService, Username};
use log::info;
use rocket::request::LenientForm;
use std::collections::HashMap;
use universe_templates::Template;

/// Helper to see if an `Option<String>` actually has a string in it
///
/// # Arguments
/// * `input` The input to check
///
/// # Returns
/// False if the input is `None`, or if it is `Some(s)` where `s` is either empty or pure whitespace.
/// True if the input is `Some(s)` where `s` contains at least one non-whitespace character
fn is_empty(input: &Option<String>) -> bool {
    match input {
        None => false,
        Some(str) => str.trim().is_empty(),
    }
}

/// Handler to attempt to register a new user and react accordingly
///
/// # Arguments
/// * `body` The form submission that we are processing
/// * `username` The username that we are registering as
/// * `user_service` The user service to work with
///
/// TODO: Implement
pub fn perform_register(
    body: LenientForm<LoginForm>,
    username: Username,
    _user_service: &dyn UserService,
) -> Template {
    let mut errors: HashMap<&str, &str> = HashMap::new();

    if is_empty(&body.email) {
        errors.insert("email", "missing");
    }

    if is_empty(&body.name) {
        errors.insert("name", "missing");
    }

    if is_empty(&body.password) {
        errors.insert("password", "missing");
    }

    if is_empty(&body.password2) {
        errors.insert("password2", "missing");
    } else if body.password != body.password2 {
        errors.insert("password2", "different");
    }

    info!("Errors processing registration form: {:?}", errors);
    Template::new("login/register.tera")
        .with_data("username", username.as_ref())
        .with_data("email", body.email.as_ref().unwrap_or(&"".to_owned()))
        .with_data("name", body.name.as_ref().unwrap_or(&"".to_owned()))
        .with_data("errors", &errors)
}
