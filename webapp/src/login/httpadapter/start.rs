use crate::users::{UserService, Username};
use universe_templates::Template;

/// Handle when the initial login form was submitted and we need to look up a username to see if we
/// are performing a Login or a Register action
pub fn start_login(username: Username, user_service: &dyn UserService) -> Template {
    let template = match user_service.username_exists(username.clone()) {
        false => "login/register.tera",
        true => "login/login.tera",
    };

    Template::new(template).with_data("username", username.as_ref())
}
