use rocket::{routes, Route};

mod home;
pub mod templates;

/// Build the routes for all of the Webapp endpoints
pub fn routes() -> Vec<Route> {
    routes![home::get_home_page]
}
