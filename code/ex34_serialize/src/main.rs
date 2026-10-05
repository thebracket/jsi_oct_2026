use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            role,
        }
    }
}

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load the file
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        // Create a file and return it
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

fn main() {
    let users = get_users();
    for (name, user) in &users {
        println!("{name}: {user:?}");
    }
}
