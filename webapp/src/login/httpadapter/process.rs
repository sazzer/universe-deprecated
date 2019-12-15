use super::{form::LoginForm, get::get_login_form, register::perform_register, start::start_login};
use crate::users::{ParseUsernameError, UserService, Username};
use rocket::{post, request::LenientForm, State};
use std::sync::Arc;
use universe_templates::Template;

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
pub fn continue_login(
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

            match body.get_action().as_ref() {
                "login" => start_login(username, user_service),
                "register" => perform_register(body, username, user_service),
                _ => start_login(username, user_service),
            }
        }
        Err(_) => get_login_form(),
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
