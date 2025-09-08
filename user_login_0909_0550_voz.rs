// user_login.rs
// A Rust program using the Rocket framework to implement a user login system.

#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::{Status, ContentType};
use rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::response::status;
use rocket::response::content::Content;

// Define a user structure with the necessary fields for login.
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

// Define a login response structure.
#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    success: bool,
    message: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![login])
}

// Define the login route and its handler.
#[post("/login", format = "json", data = "<user>")]
fn login(user: Json<User>) -> Result<Content<LoginResponse>, status::Custom<LoginResponse>> {
    // Simulate user validation logic. In a real application, you would
    // interact with a database or authentication service here.
    let credentials_valid = user.username == "admin" && user.password == "password123";
    
    if credentials_valid {
        // If credentials are valid, return a success response.
        Ok(Content(ContentType::JSON, Json(LoginResponse {
            success: true,
            message: String::from("Login successful"),
        })))
    } else {
        // If credentials are invalid, return an error response with custom status.
        Err(status::Custom(Status::Unauthorized, Json(LoginResponse {
            success: false,
            message: String::from("Invalid username or password"),
        })))
    }
}
