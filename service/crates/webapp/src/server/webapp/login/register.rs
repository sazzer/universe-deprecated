use crate::server::request_id::RequestId;
use rocket::{post, request::LenientForm, FromForm};
use serde::Serialize;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use universe_templates::Template;
use universe_users::{Password, UserData, Username};

/// The shape of the form data that the POST /login/register endpoing can accept
#[derive(FromForm, Debug, Serialize, Clone)]
pub struct RegisterForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub password2: Option<String>,
}

impl TryFrom<RegisterForm> for UserData {
    type Error = HashMap<&'static str, Option<&'static str>>;
    fn try_from(value: RegisterForm) -> Result<Self, Self::Error> {
        let passwords_match = value.password == value.password2;

        let username: Result<Username, &'static str> = value
            .username
            .ok_or("missing")
            .and_then(|u| u.parse().map_err(|_| "missing"));

        let email: Result<String, &'static str> = value
            .email
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .ok_or("missing");
        let name: Result<String, &'static str> = value
            .name
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .ok_or("missing");
        let password: Result<Password, &'static str> = value
            .password
            .ok_or("missing")
            .and_then(|p| Password::from_plaintext(p).map_err(|_| "hash_error"));
        let password2: Result<(), &'static str> = value.password2.ok_or("missing").and_then(|_| {
            if passwords_match {
                Ok(())
            } else {
                Err("different")
            }
        });

        match (username, email, name, password, password2) {
            (Ok(username), Ok(email), Ok(display_name), Ok(password), Ok(_)) => Ok(UserData {
                username,
                email,
                display_name,
                password,
            }),
            (username, email, name, password, password2) => {
                let mut errors = HashMap::new();
                errors.insert("username", username.err());
                errors.insert("email", email.err());
                errors.insert("name", name.err());
                errors.insert("password", password.err());
                errors.insert("password2", password2.err());
                Err(errors)
            }
        }
    }
}

#[post("/login/register", data = "<form>")]
#[tracing::instrument]
pub fn process_register(form: LenientForm<RegisterForm>, _request_id: RequestId) -> Template {
    let register_form = form.into_inner();
    let user: Result<UserData, HashMap<&str, Option<&str>>> = register_form.clone().try_into();

    let template = match user {
        Ok(user) => Template::new("login/register.tera"),
        Err(errors) => Template::new("login/register.tera").with_data("errors", &errors),
    };

    template
        .with_data("username", &register_form.username)
        .with_data("email", &register_form.email)
        .with_data("name", &register_form.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn test_convert_form_success() {
        let form = RegisterForm {
            username: Some("testuser".to_owned()),
            email: Some("test@example.com".to_owned()),
            name: Some("Test User".to_owned()),
            password: Some("pa55word".to_owned()),
            password2: Some("pa55word".to_owned()),
        };
        let user: UserData = form.try_into().unwrap();

        assert_that(&user.username).is_equal_to("testuser".parse::<Username>().unwrap());
        assert_that(&user.email).is_equal_to("test@example.com".to_owned());
        assert_that(&user.display_name).is_equal_to("Test User".to_owned());
        assert_that(&user.password.verify("pa55word")).is_equal_to(true);
    }

    #[test]
    fn test_convert_form_success_padded() {
        let form = RegisterForm {
            username: Some("  testuser  ".to_owned()),
            email: Some("  test@example.com  ".to_owned()),
            name: Some("  Test User  ".to_owned()),
            password: Some("  pa55word  ".to_owned()),
            password2: Some("  pa55word  ".to_owned()),
        };
        let user: UserData = form.try_into().unwrap();

        assert_that(&user.username).is_equal_to("testuser".parse::<Username>().unwrap());
        assert_that(&user.email).is_equal_to("test@example.com".to_owned());
        assert_that(&user.display_name).is_equal_to("Test User".to_owned());
        assert_that(&user.password.verify("  pa55word  ")).is_equal_to(true);
    }

    #[test]
    fn test_convert_form_all_null() {
        let form = RegisterForm {
            username: None,
            email: None,
            name: None,
            password: None,
            password2: None,
        };
        let user: Result<UserData, HashMap<&str, Option<&str>>> = form.try_into();
        let err = user.unwrap_err();

        assert_that(&err).has_length(5);
        assert_that(&err).contains_entry("username", Some("missing"));
        assert_that(&err).contains_entry("email", Some("missing"));
        assert_that(&err).contains_entry("name", Some("missing"));
        assert_that(&err).contains_entry("password", Some("missing"));
        assert_that(&err).contains_entry("password2", Some("missing"));
    }

    #[test]
    fn test_convert_form_all_spaces() {
        let form = RegisterForm {
            username: Some("    ".to_owned()),
            email: Some("    ".to_owned()),
            name: Some("    ".to_owned()),
            password: Some("    ".to_owned()),
            password2: Some("    ".to_owned()),
        };
        let user: Result<UserData, HashMap<&str, Option<&str>>> = form.try_into();
        let err = user.unwrap_err();

        assert_that(&err).has_length(5);
        assert_that(&err).contains_entry("username", Some("missing"));
        assert_that(&err).contains_entry("email", Some("missing"));
        assert_that(&err).contains_entry("name", Some("missing"));
        assert_that(&err).contains_entry("password", None);
        assert_that(&err).contains_entry("password2", None);
    }

    #[test]
    fn test_convert_form_password_mismatch() {
        let form = RegisterForm {
            username: Some("testuser".to_owned()),
            email: Some("test@example.com".to_owned()),
            name: Some("Test User".to_owned()),
            password: Some("pa55word".to_owned()),
            password2: Some("Pa55word".to_owned()),
        };
        let user: Result<UserData, HashMap<&str, Option<&str>>> = form.try_into();
        let err = user.unwrap_err();

        assert_that(&err).has_length(5);
        assert_that(&err).contains_entry("username", None);
        assert_that(&err).contains_entry("email", None);
        assert_that(&err).contains_entry("name", None);
        assert_that(&err).contains_entry("password", None);
        assert_that(&err).contains_entry("password2", Some("different"));
    }
}
