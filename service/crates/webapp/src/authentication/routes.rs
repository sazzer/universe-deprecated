use super::get::*;
use super::post::*;
use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
  routes![
    authenticate_user,
    get_access_token_required,
    get_access_token_optional
  ]
}
