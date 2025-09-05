use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::Outcome::{Forward, Success};
use rocket::Response;
use std::fmt;

// Define a general error type for our API
#[derive(Debug, PartialEq)]
enum AppError {
    NotFound,
    InternalServerError,
    InvalidInput(String),
}

// Implement `fmt::Display` for `AppError`
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::InternalServerError => write!(f, "Internal Server Error"),
            AppError::InvalidInput(ref err) => write!(f, "{}", err),
        }
    }
}

// Define a struct to represent a successful API response
#[derive(Serialize, Debug)]
struct ApiResponse<T> {
    data: T,
}

// Define a struct to represent an error API response
#[derive(Serialize, Debug)]
struct ApiError {
    message: String,
    error: String,
}

// Implement a trait to convert `AppError` into a JSON response
impl IntoResponse for AppError {
    fn into_response(self) -> rocket::response::Response<'static> {
        match self {
            AppError::NotFound => {
                Response::build()
                    .status(Status::NotFound)
                    .json(ApiError {
                        message: "Not Found".to_string(),
                        error: "Resource not found".to_string(),
                    })
            },
            AppError::InternalServerError => {
                Response::build()
                    .status(Status::InternalServerError)
                    .json(ApiError {
                        message: "Internal Server Error".to_string(),
                        error: "Internal Server Error".to_string(),
                    })
            },
            AppError::InvalidInput(err) => {
                Response::build()
                    .status(Status::BadRequest)
                    .json(ApiError {
                        message: "Invalid Input".to_string(),
                        error: err,
                    })
            },
        }
    }
}

// Implement a request guard to check if the request is valid
#[rocket::async_trait]
pub trait ApiRequestGuard {
    async fn check_request(&self) -> Result<(), AppError>;
}

// Define a simple example of an API endpoint using the formatter
#[post("/format", format = "json", data = "<request_data>")]
async fn format_request(request_data: Json<YourRequestDataType>) -> Result<Json<ApiResponse<YourRequestDataType>>, AppError> {
    // Here we assume `YourRequestDataType` is a struct defined elsewhere that represents the request data
    // We use the `ApiRequestGuard` trait to validate the request data
    let _ = request_data.check_request().await?;

    // If the request data is valid, return a JSON response with the data
    Ok(Json(ApiResponse { data: request_data.0.clone() }))
}

// Example usage of `ApiRequestGuard`
impl ApiRequestGuard for YourRequestDataType {
    // Implement the check_request method to validate the request data
    async fn check_request(&self) -> Result<(), AppError> {
        // Perform your validation logic here and return an error if necessary
        // For example:
        if self.some_field.is_empty() {
            Err(AppError::InvalidInput("Field cannot be empty".to_string()))
        } else {
            Ok(())
        }
    }
}

/// # Errors
/// This function will fail with `AppError::InvalidInput` if the request data is invalid.
/// # Panics
/// This function does not panic.

fn main() {
    rocket::build()
        .mount("/api", routes![format_request])
        .launch();
}

// Note: You will need to define your own request data type `YourRequestDataType` and implement `ApiRequestGuard` for it.
