use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct ConvertRequest {
    #[serde(crate = "rocket::serde")]
    data: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct ConvertResponse {
    #[serde(crate = "rocket::serde")]
    converted_data: String,
}

// Error type for handling conversion errors
#[derive(Debug)]
enum ConversionError {
    InvalidInput,
}

// Implementing Display to provide a user-friendly error message
impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionError::InvalidInput => write!(f, "Invalid input data provided"),
        }
    }
}

// The main function where the Rocket instance is set up
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            convert,
        ])
}

// The convert function handles the incoming request and responds with the converted data
#[get("/convert")]
fn convert(request: Json<ConvertRequest>) -> Result<Json<ConvertResponse>, ConversionError> {
    // Attempt to serialize the input data to a JSON string
    let converted_data = serde_json::to_string(&request.data)
        .map_err(|_| ConversionError::InvalidInput)?;

    // Return the converted data as a response
    Ok(Json(ConvertResponse {
        converted_data,
    }))
}
