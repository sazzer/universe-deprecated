use crate::server::request_id::RequestId;
use rocket::get;
use universe_templates::Template;

#[get("/")]
#[tracing::instrument]
pub fn get_home_page(_request_id: RequestId) -> Template {
    Template::new("home.tera")
}
