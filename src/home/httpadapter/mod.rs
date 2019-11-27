use crate::server::Template;
use rocket::get;

#[get("/")]
pub fn index() -> Template {
    Template::new("index.tera")
}
