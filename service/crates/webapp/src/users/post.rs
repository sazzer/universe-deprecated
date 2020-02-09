use super::model::User;
use crate::problem::{missing_error, unexpected_error, validation_error, Problem, ValidationError};
use rocket::{post, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
use tracing::debug;
use universe_users::*;

#[post("/users", data = "<registration>")]
pub fn register_user(
    registration: Json<Registration>,
    user_service: State<Box<dyn UserService>>,
) -> Result<Json<User>, Problem> {
    debug!("Registration: {:?}", registration);

    let user: UserData = registration
        .into_inner()
        .try_into()
        .map_err(validation_error)?;
    debug!("User Data: {:?}", user);

    let result = user_service.register_user(user)?;
    debug!("Registered user: {:?}", result);

    Ok(Json(result.into()))
}

impl From<RegisterUserError> for Problem {
    fn from(e: RegisterUserError) -> Self {
        match e {
            RegisterUserError::ValidationError(errors) => {
                let validation_errors = errors
                    .iter()
                    .map(|e| match e {
                        UserValidationError::DuplicateUsername => ValidationError {
                            r#type: "tag:universe,2020:users/validation-errors/username/duplicate"
                                .to_owned(),
                            title: "The username is already registered".to_owned(),
                            field: "username".to_owned(),
                        },
                        UserValidationError::DuplicateEmail => ValidationError {
                            r#type: "tag:universe,2020:users/validation-errors/email/duplicate"
                                .to_owned(),
                            title: "The email address is already registered".to_owned(),
                            field: "email".to_owned(),
                        },
                    })
                    .collect();
                validation_error(validation_errors)
            }
            _ => unexpected_error(),
        }
    }
}

/// Struct representing the input data for registering a new user
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Registration<'a> {
    pub username: Option<&'a str>,
    pub display_name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
}

impl<'a> TryFrom<Registration<'a>> for UserData {
    type Error = Vec<ValidationError>;

    fn try_from(input: Registration) -> Result<Self, Self::Error> {
        let username: Result<Username, ValidationError> =
            input.username.unwrap_or("").parse().map_err(|e| match e {
                UsernameParseError::Blank => missing_error("username"),
            });
        let display_name: Result<DisplayName, ValidationError> = input
            .display_name
            .unwrap_or("")
            .parse()
            .map_err(|e| match e {
                DisplayNameParseError::Blank => missing_error("displayName"),
            });
        let email: Result<EmailAddress, ValidationError> =
            input.email.unwrap_or("").parse().map_err(|e| match e {
                EmailAddressParseError::Blank => missing_error("email"),
                EmailAddressParseError::Malformed => ValidationError {
                    r#type: "tag:universe,2020:users/validation-errors/email/malformed".to_owned(),
                    title: "Email Address was malformed".to_owned(),
                    field: "email".to_owned(),
                },
            });
        let password: Result<Password, ValidationError> =
            Password::from_plaintext(input.password.unwrap_or("")).map_err(|e| match e {
                PasswordHashError::Blank => missing_error("password"),
                PasswordHashError::HashError => ValidationError {
                    r#type: "tag:universe,2020:validation-errors/password/invalid-password"
                        .to_owned(),
                    title: "The password was invalid".to_owned(),
                    field: "password".to_owned(),
                },
            });

        match (username, email, display_name, password) {
            (Ok(username), Ok(email), Ok(display_name), Ok(password)) => Ok(UserData {
                username,
                email,
                display_name,
                password,
            }),
            (username, email, name, password) => {
                let errors = vec![username.err(), email.err(), name.err(), password.err()]
                    .into_iter()
                    .filter_map(|v| v)
                    .collect();

                Err(errors)
            }
        }
    }
}
