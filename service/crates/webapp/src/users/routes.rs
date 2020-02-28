use super::get::*;
use super::patch::*;
use super::post::*;
use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
    routes![get_user_by_id, get_username, register_user, update_user]
}
