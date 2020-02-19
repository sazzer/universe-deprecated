use super::post::*;
use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
  routes![authenticate_user]
}
