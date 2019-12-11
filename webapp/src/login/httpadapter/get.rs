use rocket::get;
use universe_templates::Template;

/// Handler to display the initial login form
#[get("/login")]
pub fn get_login_form() -> Template {
    Template::new("login/start.tera")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use speculate::speculate;

    speculate! {
        describe "GET /login" {
            it "Renders the correct template" {
                let result = get_login_form();
                assert_eq!("login/start.tera", result.get_name());
                assert_eq!(json!({}), result.get_data().as_json().unwrap());
            }
        }
    }
}
