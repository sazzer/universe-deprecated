use super::templates::Template;
use crate::server::request_id::RequestId;
use rocket::get;

#[get("/")]
#[tracing::instrument]
pub fn get_home_page(_request_id: RequestId) -> Template {
    log::debug!("Hello");
    Template::new("home.tera")
}
