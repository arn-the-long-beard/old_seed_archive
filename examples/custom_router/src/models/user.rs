use crate::models::auth::AuthData;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    #[serde(flatten)]
    pub credentials: AuthData,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoggedUser {
    pub first_name: String,
    pub last_name: String,
    username: String,
    email: String,
    role: Role,
}

impl LoggedUser {
    pub fn new(first_name: String, last_name: String, username: String, email: String) -> Self {
        LoggedUser {
            first_name,
            last_name,
            username,
            email,
            role: Role::StandardUser,
        }
    }
}

impl LoggedUser {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum Role {
    StandardUser,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Role::StandardUser
    }
}
