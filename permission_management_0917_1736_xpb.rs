#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::{self, Responder, Result, status};
use std::collections::HashMap;

// Define a User model with permission data
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: i32,
    username: String,
    permissions: Vec<String>,
}

// Define an error type for permission errors
#[derive(Debug, Serialize, Deserialize)]
enum PermissionError {
    Unauthorized,
    UserNotFound,
}

// Implement Responder trait for PermissionError
impl<'r> Responder<'r, 'static> for PermissionError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> result::Result<rocket::response::Response<'static>, rocket::response::ResponderError> {
        let status = match self {
            PermissionError::Unauthorized => Status::Unauthorized,
            PermissionError::UserNotFound => Status::NotFound,
        };
        let response = rocket::response::Response::build().status(status)
            .body(rocket::serde::json::to_string(&self).unwrap());
        Ok(response)
    }
}

// Permission manager service struct
struct PermissionManager {
    users: HashMap<i32, User>,
}

impl PermissionManager {
    fn new() -> Self {
        PermissionManager {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    fn get_user(&self, user_id: i32) -> Result<&User, PermissionError> {
        self.users.get(&user_id).ok_or(PermissionError::UserNotFound)
    }

    fn check_permission(&self, user_id: i32, permission: &str) -> Result<bool, PermissionError> {
        self.get_user(user_id).map(|user| user.permissions.contains(&permission.to_string()))
    }
}

#[post("/add_user", format = "json", data = "<user>")]
fn add_user_route(permission_manager: rocket::State<PermissionManager>, user: Json<User>) -> Result<status::Custom<String>> {
    permission_manager.add_user(user.into_inner().clone());
    Ok(status::Custom(Status::Ok, "User added successfully".to_string()))
}

#[get("/check_permission/<user_id>/<permission>")]
fn check_permission_route(permission_manager: rocket::State<PermissionManager>, user_id: i32, permission: String) -> Result<Json<bool>> {
    permission_manager.check_permission(user_id, &permission).map(Json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_user_route, check_permission_route])
        .manage(PermissionManager::new())
}

// Example usage of PermissionManager
// This would typically be in a main function or tests
fn main() {
    let mut manager = PermissionManager::new();
    let user = User {
        id: 1,
        username: "testuser".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    manager.add_user(user);
    assert_eq!(manager.check_permission(1, "read").unwrap(), true);
    assert_eq!(manager.check_permission(1, "delete").unwrap(), false);
}
