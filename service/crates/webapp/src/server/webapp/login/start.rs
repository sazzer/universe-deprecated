use crate::server::request_id::RequestId;
use rocket::get;
use universe_templates::Template;

#[get("/login")]
#[tracing::instrument]
pub fn start_login(_request_id: RequestId) -> Template {
    Template::new("login/start.tera")
}
