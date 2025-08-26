 * Functionality:
 * - Handles a simple GET request at the root path.
 * - Demonstrates error handling.
 */

#[macro_use] extern crate rocket;

// Import necessary modules from Rocket.
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;

// Define a structure to hold the response data.
#[derive(serde::Serialize)]
struct ResponseData {
    message: String,
}

// Define a route that responds to GET requests at the root path.
#[get("/")]
fn index() -> Result<Json<ResponseData>, status::Custom<&'static str>> {
    // Simulate a possible error condition.
    let result = do_something_that_might_fail();
    match result {
        Ok(_) => Ok(Json(ResponseData { message: "Hello, World!".to_string() })),
        Err(e) => Err(status::Custom(Status::InternalServerError, e)),
    }
}

// Simulate an operation that might fail and return a Result.
fn do_something_that_might_fail() -> Result<(), &'static str> {
    // For demonstration purposes, this function always fails.
    // In a real-world scenario, you would perform some operation here.
    Err("Something went wrong!")
}

#[launch]
fn rocket() -> _ {
    // Configure Rocket to mount our routes and start the server.
    rocket::build()
        .mount("/", routes![index])
}
