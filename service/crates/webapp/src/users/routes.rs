use super::get::*;
use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![get_user_by_id]
}
