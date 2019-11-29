use crate::server::Template;
use log::info;
use rocket::{get, post, request::LenientForm, routes, FromForm, Route};
use serde::Serialize;
use std::collections::HashMap;

/// Handler to display the initial login form
#[get("/login")]
fn get_login_form() -> Template {
    Template::new("login/start.tera")
}

/// Form data to represent the various login forms that we might have.
///
/// This needs to cover the initial login form, containing only the username, as well as the forms
/// on the Register page and on the Login page.
#[derive(FromForm, Debug, Serialize)]
struct LoginForm {
    action: Option<String>,
    username: Option<String>,
    email: Option<String>,
    name: Option<String>,
    password: Option<String>,
    password2: Option<String>,
}

/// Handle when the initial login form was submitted and we need to look up a username to see if we
/// are performing a Login or a Register action
fn start_login(body: LenientForm<LoginForm>) -> Template {
    let known_user = body
        .username
        .clone()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v == "sazzer");

    let template = match known_user {
        Some(false) => "login/register.tera",
        Some(true) => "login/login.tera",
        None => "login/start.tera",
    };

    Template::new(template).with_data("username", body.username.as_ref().unwrap())
}

/// Handler to attempt to log the user in and react accordingly
///
/// # Arguments
/// * `body` The form submission that we are processing
///
/// TODO: Implement
fn perform_login(body: LenientForm<LoginForm>) -> Template {
    start_login(body)
}

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
///
/// TODO: Implement
fn perform_register(body: LenientForm<LoginForm>) -> Template {
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
        .with_data("username", body.username.as_ref().unwrap())
        .with_data("email", body.email.as_ref().unwrap_or(&"".to_owned()))
        .with_data("name", body.name.as_ref().unwrap_or(&"".to_owned()))
        .with_data("errors", &errors)
}

/// Controller for processing a submitted login form.
///
/// This works out which method to delegate to in order to handle the actual work.
#[post("/login", data = "<body>")]
fn continue_login(body: LenientForm<LoginForm>) -> Template {
    if body.action == Some("login".to_owned()) {
        perform_login(body)
    } else if body.action == Some("register".to_owned()) {
        perform_register(body)
    } else {
        start_login(body)
    }
}

pub fn routes() -> Vec<Route> {
    routes![get_login_form, continue_login]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_show_login_form() {
        let result = get_login_form();
        assert_eq!("login/start.tera", result.get_name());
        assert_eq!(json!({}), result.get_data().as_json().unwrap());
    }

    #[test]
    fn test_perform_register_all_missing() {
        let data = LoginForm {
            action: Some("register".to_owned()),
            username: Some("testuser".to_owned()),
            email: Some("".to_owned()),
            name: Some("".to_owned()),
            password: Some("".to_owned()),
            password2: Some("".to_owned()),
        };
        let result = perform_register(LenientForm(data));
        assert_eq!("login/register.tera", result.get_name());
        assert_eq!(
            json!({
                "username": "testuser",
                "email": "",
                "name": "",
                "errors": {
                    "email": "missing",
                    "name":"missing",
                    "password":"missing",
                    "password2":"missing",
                },
            }),
            result.get_data().as_json().unwrap()
        );
    }

    #[test]
    fn test_perform_register_password_mismatch() {
        let data = LoginForm {
            action: Some("register".to_owned()),
            username: Some("testuser".to_owned()),
            email: Some("testuser@example.com".to_owned()),
            name: Some("Test User".to_owned()),
            password: Some("pa55word".to_owned()),
            password2: Some("Pa55word".to_owned()),
        };
        let result = perform_register(LenientForm(data));
        assert_eq!("login/register.tera", result.get_name());
        assert_eq!(
            json!({
                "username": "testuser",
                "email": "testuser@example.com",
                "name": "Test User",
                "errors": {
                    "password2":"different",
                },
            }),
            result.get_data().as_json().unwrap()
        );
    }
}
