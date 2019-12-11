mod form;
mod get;
mod process;
mod register;
mod start;

use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![get::get_login_form, process::continue_login]
}
