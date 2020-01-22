use super::model::User;
use rocket::{get, State};
use rocket_contrib::json::Json;
use universe_users::{UserID, UserService};

#[get("/users/<_user_id>")]
pub fn get_user_by_id(_user_id: String, user_service: State<Box<dyn UserService>>) -> Json<User> {
    let user_id: UserID = "A32A117E-EBF2-48D8-9735-BE5F5B041A35".parse().unwrap();
    user_service.get_user_by_id(&user_id);

    let user = User {
        id: "user_id".to_owned(),
        username: "testuser".to_owned(),
        email: Some("test@example.com".to_owned()),
        display_name: "Test User".to_owned(),
    };

    Json(user)
}
