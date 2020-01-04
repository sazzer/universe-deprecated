use super::templates::Template;
use rocket::get;

#[get("/")]
#[tracing::instrument]
pub fn get_home_page() -> Template {
    Template::new("home.tera")
}
