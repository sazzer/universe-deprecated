use crate::users::{ParseUsernameError, UserService, Username};
use log::info;
use rocket::{get, post, request::LenientForm, routes, FromForm, Route, State};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use universe_templates::Template;

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
fn start_login(username: Username, user_service: &dyn UserService) -> Template {
    let template = match user_service.username_exists(username.clone()) {
        false => "login/register.tera",
        true => "login/login.tera",
    };

    Template::new(template).with_data("username", &username.0)
}

/// Handler to attempt to register a new user and react accordingly
///
/// # Arguments
/// * `body` The form submission that we are processing
/// * `username` The username that we are registering as
/// * `user_service` The user service to work with
///
/// TODO: Implement
fn perform_register(
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
        .with_data("username", &username.0)
        .with_data("email", body.email.as_ref().unwrap_or(&"".to_owned()))
        .with_data("name", body.name.as_ref().unwrap_or(&"".to_owned()))
        .with_data("errors", &errors)
}

/// Controller for processing a submitted login form.
///
/// This works out which method to delegate to in order to handle the actual work.
///
/// If we don't even have a username then we'll just show the start form again.
/// If we have a username but don't have an action then delegate to `start_login` to work out if
/// we are starting to login or register.
/// If we have a username and an action then we are actually performing either a login or register
/// as specified by the action.
///
/// # Arguments
/// * `body` The form body that was submitted
/// * `user_service` The user service that we want to use for various things
///
/// # Returns
/// The result of the form submission.
#[post("/login", data = "<body>")]
fn continue_login(
    body: LenientForm<LoginForm>,
    user_service: State<Arc<dyn UserService>>,
) -> Template {
    let username: Result<Username, ParseUsernameError> = body
        .username
        .clone()
        .map(|username| username.parse())
        .unwrap_or(Err(ParseUsernameError::BlankUsername));

    match username {
        Ok(username) => {
            let user_service = user_service.inner().as_ref();
            if body.action == Some("login".to_owned()) {
                start_login(username, user_service)
            } else if body.action == Some("register".to_owned()) {
                perform_register(body, username, user_service)
            } else {
                start_login(username, user_service)
            }
        }
        Err(_) => get_login_form(),
    }
}

pub fn routes() -> Vec<Route> {
    routes![get_login_form, continue_login]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::users::service::MockUserService;
    use serde_json::json;
    use speculate::speculate;
    use std::sync::Arc;

    speculate! {
        describe "GET /login" {
            it "Renders the correct template" {
                let result = get_login_form();
                assert_eq!("login/start.tera", result.get_name());
                assert_eq!(json!({}), result.get_data().as_json().unwrap());
            }
        }

        describe "POST /login" {
            describe "With no action provided" {
                it "Shows the initial login form if no username was provided" {
                    let data = LoginForm {
                        action: None,
                        username: None,
                        email: None,
                        name: None,
                        password: None,
                        password2: None,
                    };

                    let user_service = MockUserService::new();
                    let rocket = rocket::ignite().manage(Arc::new(user_service) as Arc<dyn UserService>);
                    let result = continue_login(LenientForm(data), rocket::State::from(&rocket).unwrap());

                    assert_eq!("login/start.tera", result.get_name());
                    assert_eq!(json!({}), result.get_data().as_json().unwrap());
                }

                it "Shows the register form if the username was unknown" {
                    let data = LoginForm {
                        action: None,
                        username: Some("testuser".to_owned()),
                        email: None,
                        name: None,
                        password: None,
                        password2: None,
                    };

                    let mut user_service = MockUserService::new();
                    user_service
                        .expect_username_exists()
                        .with(mockall::predicate::eq(Username("testuser".to_owned())))
                        .times(1)
                        .return_const(false);

                    let rocket = rocket::ignite().manage(Arc::new(user_service) as Arc<dyn UserService>);
                    let result = continue_login(LenientForm(data), rocket::State::from(&rocket).unwrap());

                    assert_eq!("login/register.tera", result.get_name());
                    assert_eq!(
                        json!({
                            "username": "testuser",
                        }),
                        result.get_data().as_json().unwrap()
                    );
                }

                it "Shows the login form if the username was known" {
                    let data = LoginForm {
                        action: None,
                        username: Some("testuser".to_owned()),
                        email: None,
                        name: None,
                        password: None,
                        password2: None,
                    };

                    let mut user_service = MockUserService::new();
                    user_service
                        .expect_username_exists()
                        .with(mockall::predicate::eq(Username("testuser".to_owned())))
                        .times(1)
                        .return_const(true);

                    let rocket = rocket::ignite().manage(Arc::new(user_service) as Arc<dyn UserService>);
                    let result = continue_login(LenientForm(data), rocket::State::from(&rocket).unwrap());

                    assert_eq!("login/login.tera", result.get_name());
                    assert_eq!(
                        json!({
                            "username": "testuser",
                        }),
                        result.get_data().as_json().unwrap()
                    );
                }
            }

            describe "With an action of 'register'" {
                it "Shows errors if no fields are populated" {
                    let data = LoginForm {
                        action: Some("register".to_owned()),
                        username: Some("testuser".to_owned()),
                        email: Some("".to_owned()),
                        name: Some("".to_owned()),
                        password: Some("".to_owned()),
                        password2: Some("".to_owned()),
                    };

                    let user_service = MockUserService::new();
                    let rocket = rocket::ignite().manage(Arc::new(user_service) as Arc<dyn UserService>);
                    let result = continue_login(LenientForm(data), rocket::State::from(&rocket).unwrap());

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
                it "Shows errors if the passwords didn't match" {
                    let data = LoginForm {
                        action: Some("register".to_owned()),
                        username: Some("testuser".to_owned()),
                        email: Some("testuser@example.com".to_owned()),
                        name: Some("Test User".to_owned()),
                        password: Some("pa55word".to_owned()),
                        password2: Some("Pa55word".to_owned()),
                    };

                    let user_service = MockUserService::new();
                    let rocket = rocket::ignite().manage(Arc::new(user_service) as Arc<dyn UserService>);
                    let result = continue_login(LenientForm(data), rocket::State::from(&rocket).unwrap());

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
        }
    }
}
