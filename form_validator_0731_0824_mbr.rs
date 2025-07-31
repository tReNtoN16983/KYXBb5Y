use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value as JsonValue;
use rocket::State;
use rocket::outcome::Outcome;
use rocket::request::{self, Request, FromRequest};
use rocket::response::status;
use rocket::response::Response;
use rocket::serde;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::http::RawStr;
use rocket::outcome::IntoOutcome;
use rocket::data::Data;
use rocket::handler::Handler;
use rocket::request::{FlashMessage, Outcome as RequestOutcome};
use rocket::Config;
use rocket::Rocket;
use rocket::Route;
use rocket::Request;
use rocket::Response;
use rocket::State;
use rocket::Outcome;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value as JsonValue;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::http::Status;
use std::ops::Deref;
use std::str::FromStr;
use std::result::Result;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;
use serde::de::DeserializeOwned;
use rocket::http::RawStr;
use rocket::data::Data;
use rocket::handler::Handler;
use rocket::outcome::Outcome;
use rocket::request::{FlashMessage, Outcome as RequestOutcome};
use rocket::Config;
use rocket::Rocket;
use rocket::Route;
use rocket::Request;
use rocket::Response;
use rocket::State;
use rocket::Outcome;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value as JsonValue;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::http::Status;
use std::ops::Deref;
use std::str::FromStr;
use std::result::Result;
use std::fmt;
use std::error::Error;
use std::collections::HashMap;
use serde::de::DeserializeOwned;

// Define a custom error type for form validation errors
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation error on field {}: {}", self.field, self.message)
    }
}

impl Error for ValidationError {}

// A trait to represent a form that can be validated
pub trait ValidatableForm: Sized {
    type ValidationError: Error + Send + Sync;
    fn validate(&self) -> Result<(), Self::ValidationError>;
}

// Implement the ValidatableForm trait for a struct
#[derive(Serialize, Deserialize)]
pub struct MyForm {
    name: String,
    age: u32,
}

impl ValidatableForm for MyForm {
    type ValidationError = ValidationError;
    fn validate(&self) -> Result<(), Self::ValidationError> {
        if self.name.is_empty() {
            Err(ValidationError {
                field: "name".to_string(),
                message: "Name cannot be empty".to_string(),
            })
        } else if self.age < 18 {
            Err(ValidationError {
                field: "age".to_string(),
                message: "Age must be at least 18".to_string(),
            })
        } else {
            Ok(())
        }
    }
}

// Define a custom error handler for validation errors
#[catch(400)]
fn handle_validation_error(error: ValidationError, _req: &Request<'_>) -> Json<JsonValue> {
    Json(json!({
        "errors": vec![error],
    }))
}

#[launch]
fn rocket() -> Rocket<'static> {
    rocket::build()
        .mount("/", routes![submit_form])
        .register.catcher(catchers![handle_validation_error])
}

// Define a route to handle form submission
#[post("/form", format = "json", data = "<form>")]
fn submit_form(form: Json<MyForm>) -> Result<status::Created<&'static str>, status::BadRequest<Json<JsonValue>>> {
    match form.0.validate() {
        Ok(_) => Ok(status::Created("Form submitted successfully".into())),
        Err(error) => Err(status::BadRequest(Some(Json(json!({
            "errors": vec![error],
        })))),
    }
}

fn main() {
    rocket().launch();
}
