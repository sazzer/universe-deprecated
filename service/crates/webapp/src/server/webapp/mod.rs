use rocket::{routes, Route};

mod home;
mod login;

/// Build the routes for all of the Webapp endpoints
pub fn routes() -> Vec<Route> {
    let mut result = routes![home::get_home_page];
    result.extend(login::routes());
    result
}
