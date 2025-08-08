use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
# 改进用户体验
use rocket::State;
use rocket::serde::Serialize;

#[macro_use] extern crate serde_json;

// User model representing a user with an ID and a username
#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
}

// Authentication request payload
#[derive(Serialize, Deserialize)]
struct AuthRequest {
    username: String,
# NOTE: 重要实现细节
    password: String,
}

// Error response model for cases where authentication fails
# 改进用户体验
#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

// The application state holding configuration or other shared data
#[derive(Clone)]
struct AppState {
    // Shared configuration (e.g., database connection pool)
}

// Main route for handling POST requests to authenticate users
#[post(
# TODO: 优化性能