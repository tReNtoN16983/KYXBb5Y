// access_control.rs
// This module provides access control functionalities using Rocket framework.

use rocket::http::{Status, StatusClass};
use rocket::response::{status};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use std::collections::HashSet;

// Define a struct to hold user roles
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserRole {
    roles: HashSet<String>,
}

// Define a custom error type for unauthorized access
#[derive(Debug, Serialize)]
struct UnauthorizedError {
    status: Status,
    message: String,
}

#[macro_export]
macro_rules! user_role_state {
    () => {
        State::<Mutex<UserRole>>::from(Mutex::new(UserRole { roles: HashSet::new() }))
    };
}

// A fairing to inject the user's role into the request
#[rocket::fairing]
fn user_role() -> impl rocket::fairing::Fairing {
    rocket::fairing::AdHoc::on_attach("UserRole", |rocket| {
        Ok(rocket.manage(Mutex::new(UserRole { roles: HashSet::new() })))
    })
}

// A request guard to check user's role
#[rocket::async_trait]
pub trait Authorize {
    async fn has_access(&self) -> bool;
}

// Implement the Authorize trait for any type that can provide a user role
impl<'r> Authorize for rocket::Request<'r> {
    async fn has_access(&self) -> bool {
        let user_role = self.guard::<State<Mutex<UserRole>>>().await.unwrap();
        let user_role = user_role.lock().unwrap();
        // Check if the user has required role
        user_role.roles.contains("admin")
    }
}

// Define a route that requires admin role access
#[get("/secure")]
fn secure_endpoint() -> Json<UnauthorizedError> {
    if let Ok(req) = rocket::request::Request::current().await {
        if req.has_access().await {
            Json(UnauthorizedError { status: Status::Ok, message: "Welcome admin!".to_string() })
        } else {
            Json(UnauthorizedError { status: Status::Unauthorized, message: "Unauthorized access!".to_string() })
        }
    } else {
        Json(UnauthorizedError { status: Status::InternalServerError, message: "Internal Server Error!".to_string() })
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![secure_endpoint])
        .attach(user_role())
}
