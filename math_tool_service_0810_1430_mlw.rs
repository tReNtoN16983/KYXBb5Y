 * maintainable, and extensible, following Rust best practices and proper error handling.
 */

use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use std::ops::{Add, Sub, Mul, Div};

#[macro_use]
extern crate rocket;

// Define a structure to represent the request payload for arithmetic operations.
#[derive(Serialize, Deserialize)]
struct OperationRequest {
    a: f64,
    b: f64,
    operation: String,
}

// Define a structure to represent the response payload for the arithmetic operations.
#[derive(Serialize, Deserialize)]
struct OperationResponse {
    result: f64,
    operation: String,
}

// Define the errors that can occur during the arithmetic operations.
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    InvalidOperation,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::InvalidOperation => write!(f, "Invalid operation requested"),
        }
    }
}

// Implement the arithmetic operations.
impl OperationRequest {
    #[must_use]
    fn perform_operation(self) -> Result<OperationResponse, MathError> {
        match self.operation.as_str() {
            "add" => Ok(OperationResponse {
                result: self.a + self.b,
                operation: self.operation,
            }),
            "subtract" => Ok(OperationResponse {
                result: self.a - self.b,
                operation: self.operation,
            }),
            "multiply" => Ok(OperationResponse {
                result: self.a * self.b,
                operation: self.operation,
            }),
            "divide" => {
                if self.b == 0.0 {
                    Err(MathError::DivisionByZero)
                } else {
                    Ok(OperationResponse {
                        result: self.a / self.b,
                        operation: self.operation,
                    })
                }
            }
            _ => Err(MathError::InvalidOperation),
        }
    }
}

// Define the route for performing an arithmetic operation.
#[get("/calculate")]
fn calculate(op: Json<OperationRequest>) -> Result<Json<OperationResponse>, String> {
    match op.perform_operation() {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(e.to_string()),
    }
}

// Define the main function to launch the Rocket server.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![calculate])
}
