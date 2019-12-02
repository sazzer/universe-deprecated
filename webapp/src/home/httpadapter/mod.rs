use crate::server::Template;
use rocket::{get, routes, Route};

#[get("/")]
fn index() -> Template {
    Template::new("index.tera")
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
