use crate::model::*;
use crate::service::repository::UserRepository;
use universe_database::Database;

impl UserRepository for Database {
    fn get_user_by_id(&self, _user_id: &UserID) -> Option<UserEntity> {
        let mut client = self.client().unwrap();
        client.query("SELECT 1", &[]).unwrap();
        None
    }
}
