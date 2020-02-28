use crate::authentication::model::{AccessToken, AuthenticatedUser};
use crate::problem::{Problem, ValidationError};
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

    let user: UserData = registration.into_inner().try_into()?;
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
        let username: Result<Username, ValidationError> = input
            .username
            .unwrap_or("")
            .parse()
            .map_err(|e: UsernameParseError| e.into());
        let display_name: Result<DisplayName, ValidationError> = input
            .display_name
            .unwrap_or("")
            .parse()
            .map_err(|e: DisplayNameParseError| e.into());
        let email: Result<EmailAddress, ValidationError> = input
            .email
            .unwrap_or("")
            .parse()
            .map_err(|e: EmailAddressParseError| e.into());
        let password: Result<Password, ValidationError> =
            Password::from_plaintext(input.password.unwrap_or(""))
                .map_err(|e: PasswordHashError| e.into());

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
