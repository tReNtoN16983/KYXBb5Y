#[macro_use]
# 增强安全性
extern crate rocket;

use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use rocket::Request;

// This function sanitizes input to prevent XSS attacks.
// It uses a simple approach by stripping out HTML tags.
# 增强安全性
fn sanitize_input(input: &str) -> String {
    input.replace("<<", "&lt;").replace(">>