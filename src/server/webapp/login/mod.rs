use rocket::{routes, Route};

mod start;

pub fn routes() -> Vec<Route> {
    routes![start::start_login]
}
