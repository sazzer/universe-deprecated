use super::search::*;
use rocket::{routes, Route};

pub fn routes() -> Vec<Route> {
  routes![search_worlds]
}
