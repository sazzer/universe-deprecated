use super::templates::Template;
use rocket::get;

#[get("/")]
pub fn get_home_page() -> Template {
    Template::new("home.tera")
}
