use crate::authentication::model::{AccessToken, AuthenticatedUser};
use crate::problem::{missing_error, unexpected_error, validation_error, Problem, ValidationError};
use crate::request_id::RequestId;
use rocket::{post, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
use tracing::debug;
use universe_authentication::{encoder::AccessTokenEncoder, AccessTokenFactory};
use universe_users::*;

#[post("/users", data = "<registration>")]
#[tracing::instrument(skip(user_service, access_token_factory, access_token_encoder))]
pub fn register_user(
    _request_id: RequestId,
    registration: Json<Registration>,
    user_service: State<Box<dyn UserService>>,
    access_token_factory: State<AccessTokenFactory>,
    access_token_encoder: State<AccessTokenEncoder>,
) -> Result<Json<AuthenticatedUser>, Problem> {
    debug!("Registration: {:?}", registration);

    let user: UserData = registration
        .into_inner()
        .try_into()
        .map_err(validation_error)?;
    debug!("User Data: {:?}", user);

    let result = user_service.register_user(user)?;
    debug!("Registered user: {:?}", result);

    let access_token = access_token_factory.build(&result.identity.id);
    debug!(
        "Access Token for user {:?}: {:?}",
        result.identity.id, access_token
    );

    Ok(Json(AuthenticatedUser {
        user: result.into(),
        access_token: AccessToken {
            token: access_token_encoder.encode(&access_token).unwrap(),
            expiry: access_token.expires,
        },
    }))
}

impl From<RegisterUserError> for Problem {
    fn from(e: RegisterUserError) -> Self {
        match e {
            RegisterUserError::ValidationError(errors) => {
                validation_error(errors.iter().map(|e| e.into()).collect())
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
