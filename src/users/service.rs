use super::Username;
use log::debug;
use mockall::automock;

/// Trait to represent the User Service for interacting with users
#[automock]
pub trait UserService: Send + Sync {
    /// Check if a user with the given username already exists.
    ///
    /// # Arguments
    /// * `username` The username to look for
    ///
    /// # Returns
    /// True if the username is already known to the system. False if it's new.
    fn username_exists(&self, username: Username) -> bool;
}

/// The actual user service implementation
pub struct UserServiceImpl {}

impl UserServiceImpl {
    pub fn new() -> Self {
        UserServiceImpl {}
    }
}

impl UserService for UserServiceImpl {
    fn username_exists(&self, username: Username) -> bool {
        debug!("Looking up username {:?}", username);

        match username.0.as_str() {
            "sazzer" => true,
            _ => false,
        }
    }
}
