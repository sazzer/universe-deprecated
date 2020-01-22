use super::model::User;
use rocket::get;
use rocket_contrib::json::Json;

#[get("/users/<user_id>")]
pub fn get_user_by_id(user_id: String) -> Json<User> {
    let user = User {
        id: user_id,
        username: "testuser".to_owned(),
        email: Some("test@example.com".to_owned()),
        display_name: "Test User".to_owned(),
    };

    Json(user)
}
