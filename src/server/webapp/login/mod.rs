use rocket::{routes, Route};

mod process;
mod start;

pub fn routes() -> Vec<Route> {
    routes![start::start_login, process::process_login]
}