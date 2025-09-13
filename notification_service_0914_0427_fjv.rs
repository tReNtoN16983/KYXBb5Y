use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

// Define a struct to represent a notification
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Notification {
    title: String,
    message: String,
}

// Define an error type for our application
#[derive(Debug)]
enum NotificationError {
    InvalidInput(String),
    InternalError,
}

impl<'r> From<&'r str> for NotificationError {
    fn from(s: &'r str) -> Self {
        NotificationError::InvalidInput(s.to_string())
    }
}

impl Error for NotificationError {}

// Define a handler for the notification endpoint
#[get("/notify")]
fn notify(notification: Json<Notification>) -> Result<status::Accepted<Json<Value>>, status::BadRequest<Json<Value>>> {
    // Check if the notification has both title and message
    if notification.title.is_empty() || notification.message.is_empty() {
        return Err(status::BadRequest(Some(Json(json!({
            "error": "Missing title or message"
        }))))
    }

    // Here you would add logic to actually send the notification (e.g., to a messaging service)
    // For the sake of this example, we'll just return an acceptance status
    
    Ok(status::Accepted(Some(Json(json!({
        "status": "Notification sent successfully"
    }))))
}

// Define the Rocket launch configuration
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![notify])
}

// Add any necessary documentation comments
/// This is a simple message notification system API using Rocket.
/// It's designed to be easily extensible and maintainable, with clear error handling and documentation.
/// # Errors
/// - `InvalidInput`: Returned when the input notification is missing a title or message.
/// - `InternalError`: Not used in this example, but could be used for internal server errors.
