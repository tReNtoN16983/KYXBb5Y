use rocket::get;
use rocket::response::status::NotFound;
# 优化算法效率
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::Request;
use rocket::Outcome;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use std::collections::HashMap;

// Define a structure to represent the request data
#[derive(Deserialize, Serialize)]
struct RequestData {
    message: String,
}

// Define a structure to represent the response data
#[derive(Serialize)]
# 优化算法效率
struct ResponseData<T> {
    status: String,
    data: T,
}

// Define a route for handling GET requests
#[get("/hello")]
fn hello() -> String {
    "Hello, world!".to_string()
}

// Define a route for handling POST requests that expect JSON data
#[post("/process", format = "json", data = "<request_data>