use crate::server::request_id::RequestId;
use rocket::{post, request::LenientForm, FromForm, State};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use universe_templates::Template;
use universe_users::{UserService, Username};

/// The shape of the form data that the POST /login endpoing can accept
#[derive(FromForm, Debug)]
pub struct LoginForm {
    pub username: Option<String>,
}

impl TryFrom<LoginForm> for Username {
    type Error = HashMap<&'static str, &'static str>;
    fn try_from(value: LoginForm) -> Result<Self, Self::Error> {
        value
            .username
            .ok_or("missing")
            .and_then(|u| u.parse().map_err(|_| "missing"))
            .map_err(|e| {
                let mut errors = HashMap::new();
                errors.insert("username", e);
                errors
            })
    }
}

#[post("/login", data = "<form>")]
#[tracing::instrument(skip(user_service))]
pub fn process_login(
    form: LenientForm<LoginForm>,
    user_service: State<Arc<dyn UserService>>,
    _request_id: RequestId,
) -> Template {
    let login_form = form.into_inner();
    let username: Result<Username, HashMap<&'static str, &'static str>> = login_form.try_into();
    tracing::debug!("Username parsed from form: {:?}", username);

    match username {
        Ok(username) => {
            let known_user = user_service.username_exists(&username);
            match known_user {
                true => Template::new("login/login.tera").with_data("username", &username),
                false => Template::new("login/register.tera").with_data("username", &username),
            }
        }
        Err(e) => Template::new("login/start.tera").with_data("errors", &e),
    }
}
