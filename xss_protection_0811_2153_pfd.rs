#[macro_use]
extern crate rocket;

// Define a module for utility functions.
mod utils;

use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::env;
use utils::sanitize_input;

// Define a data structure to hold user input.
#[derive(FromForm, Serialize, Deserialize)]
struct UserInput {
    message: String,
}

// Define the main function that runs the Rocket application.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit])
}

// Serve the initial HTML form page.
#[get("/")]
fn index() -> &'static str {
    "<form action='/submit' method='post'>\
  <p>Enter a message:</p>\
  <input type='text' name='message'>\
  <input type='submit' value='Submit'>\
</form>"
}

// Handle the form submission and sanitize the input to prevent XSS.
#[post("/submit", data = "<input>")]
fn submit(input: Json<UserInput>) -> Result<String, Status> {
    // Sanitize the user input.
    let sanitized_message = sanitize_input(&input.message);

    // If sanitization fails, return an error.
    if sanitized_message.is_empty() {
        Err(Status::BadRequest)
    } else {
        // Return the sanitized message as a success response.
        Ok(format!("<p>Message received: {}</p>", sanitized_message))
    }
}

// Define a module for utility functions.
mod utils {
    // This function sanitizes the user input to prevent XSS attacks.
    // It replaces potentially malicious characters with safe alternatives.
    pub fn sanitize_input(input: &str) -> String {
        input
            .replace("<script", "&lt;script")
            .replace("<script>