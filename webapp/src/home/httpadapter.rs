use rocket::{get, routes, Route};
use universe_templates::Template;

#[get("/")]
fn index() -> Template {
    Template::new("index.tera")
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
