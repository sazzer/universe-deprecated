use crate::server::{request_id::RequestId, webapp::templates::Template};
use rocket::get;

#[get("/login")]
#[tracing::instrument]
pub fn start_login(_request_id: RequestId) -> Template {
    Template::new("login/start.tera")
}
