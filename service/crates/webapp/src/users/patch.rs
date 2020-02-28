use super::model::User;
use super::problems::{unknown_user_problem, ValidationErrors};
use crate::problem::{Problem, ValidationError};
use crate::request_id::RequestId;
use rocket::{patch, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use tracing::{debug, warn};
use universe_users::*;

#[patch("/users/<user_id>", data = "<patch_data>")]
#[tracing::instrument(skip(user_service, access_token_factory, access_token_encoder))]
pub fn update_user(
    _request_id: RequestId,
    user_id: String,
    patch_data: Json<PatchData>,
    user_service: State<Box<dyn UserService>>,
) -> Result<Json<User>, Problem> {
    debug!("Patch Data: {:?}", patch_data);

    let user_id: UserID = user_id.parse().map_err(|e| {
        warn!("Invalid User ID: {}", e);
        unknown_user_problem()
    })?;

    let user = user_service.update_user(&user_id, &mut |mut user| {
        debug!("Patching user details");

        let display_name_error = patch_data.display_name.and_then(|display_name| {
            display_name
                .parse()
                .map_err(|e: DisplayNameParseError| e.into())
                .map(|display_name: DisplayName| {
                    user.display_name = display_name.clone();
                })
                .err()
        });

        let email_error = patch_data.email.and_then(|email| {
            email
                .parse()
                .map_err(|e: EmailAddressParseError| e.into())
                .map(|email: EmailAddress| {
                    user.email = email;
                })
                .err()
        });

        let password_error = patch_data.password.and_then(|password| {
            Password::from_plaintext(password)
                .map_err(|e: PasswordHashError| e.into())
                .map(|password: Password| {
                    user.password = password;
                })
                .err()
        });

        match (display_name_error, email_error, password_error) {
            (None, None, None) => Ok(user),
            (email, name, password) => {
                let errors: Vec<ValidationError> = vec![email, name, password]
                    .into_iter()
                    .filter_map(|v| v)
                    .collect();
                warn!("Error patching user: {:?}", errors);
                Err(Box::new(ValidationErrors { errors }))
            }
        }
    })?;

    Ok(Json(user.into()))
}

/// Struct representing the input data for updating a user
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatchData<'a> {
    pub display_name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
}
