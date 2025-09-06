 * Features:
 * - Analyzes text files and provides statistics such as word count, line count, and character count.
 * - Handles errors gracefully and provides user-friendly messages.
 * - Designed for maintainability and extensibility.
 */

use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::State;
use rocket::http::Status;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

// Define a structure to hold the analysis results.
#[derive(Debug, Serialize, Deserialize)]
struct AnalysisResult {
    file_path: String,
    word_count: usize,
    line_count: usize,
    character_count: usize,
}

// Define an error type for file operations.
#[derive(Debug)]
enum FileAnalysisError {
    NotFound(String),
    PermissionDenied(String),
    IoError(io::Error),
}

// Implement Display trait for FileAnalysisError to provide user-friendly error messages.
impl std::fmt::Display for FileAnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileAnalysisError::NotFound(path) => write!(f, "File not found: {}", path),
            FileAnalysisError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            FileAnalysisError::IoError(error) => write!(f, "IO error: {}", error),
        }
    }
}

// Define a Rocket config.
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![analyze_file])
        .launch()
        .await
        .expect("Failed to launch Rocket server