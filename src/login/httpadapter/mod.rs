use crate::server::Template;
use log::info;
use rocket::{get, post, request::Form, routes, FromForm, Route};
use serde::Serialize;

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
fn start_login(body: Form<LoginForm>) -> Template {
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

/// Controller for processing a submitted login form.
///
/// This works out which method to delegate to in order to handle the actual work.
#[post("/login", data = "<body>")]
fn continue_login(body: Form<LoginForm>) -> Template {
    info!("Received login request: {:?}", body);

    match body.action {
        _ => start_login(body),
    }
}

pub fn routes() -> Vec<Route> {
    routes![get_login_form, continue_login]
}
