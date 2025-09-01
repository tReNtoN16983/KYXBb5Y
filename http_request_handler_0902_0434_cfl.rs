adheres to Rust best practices for maintainability and scalability.
*/
fn main() {
    // Launch the Rocket web server
    rocket::ignite()
        .mount("/", routes![
            index,
            hello,
        ])
        .launch();
}

// Define the routes
#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request,Outcome};

// The index route responds with a simple welcome message
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Rust and Rocket HTTP Request Handler!"
}

// The hello route responds with a greeting
#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

// Custom error type for handling errors
#[derive(Debug)]
enum MyError {
    NotFound,
}

// Implementing the error handler
#[error("404 Not Found")]
fn not_found_error() -> Json<MyError> {
    Json(MyError::NotFound)
}

// Custom error handler for our MyError type
#[catch(404)]
fn not_found_handler() -> Json<MyError> {
    not_found_error()
}

// Define a custom request guard for handling errors
struct MyErrorGuard;

impl<'r> FromRequest<'r> for MyErrorGuard {
    type Error = MyError;

    fn from_request(request: &'r Request, _: &'r rocket::Data) -> request::Outcome<Self, Self::Error> {
        match request.uri().path() {
            // If the path is not recognized, return an error
            _ if request.uri().path().is_empty() => Outcome::Failure((Status::NotFound, MyError::NotFound)),
            _ => Outcome::Success(MyErrorGuard),
        }
    }
}

// Register the custom error handler and request guard
#[macro_export]
macro_rules! routes {
    ($($route:tt)*) => {
        vec![$($route)*]
    }
}