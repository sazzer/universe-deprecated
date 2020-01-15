use rocket::{routes, Route};

mod problem;
mod users;

/// Build the routes for all of the REST API endpoints
pub fn routes() -> Vec<Route> {
    routes![users::get_user_by_id]
}
