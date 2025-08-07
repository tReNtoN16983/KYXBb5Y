// text_file_analyzer.rs
// A program that analyzes the content of a text file using Rust and Rocket framework.

#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Form, Serialize, Deserialize};
use rocket::response::status::BadRequest;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

// Define a struct to represent the analysis request data
#[derive(Deserialize)]
pub struct AnalysisRequest {
    file_path: String,
}

// Define a struct to represent the analysis results
#[derive(Serialize)]
pub struct AnalysisResults {
    pub word_count: usize,
    pub line_count: usize,
# 改进用户体验
    pub character_count: usize,
}

// Define a struct for the error response
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Analyze the content of the text file
fn analyze_file<P: AsRef<Path>>(path: P) -> io::Result<AnalysisResults> {
    let path = path.as_ref();
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
# 改进用户体验
    file.read_to_string(&mut contents)?;

    let word_count = contents.split_whitespace().count();
    let line_count = contents.lines().count();
    let character_count = contents.chars().count();
# 改进用户体验

    Ok(AnalysisResults { word_count, line_count, character_count })
}

// The main function where the program starts
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/analyze", rocket::routes![analyze_file_endpoint])
}

// The endpoint to handle file analysis requests
#[post("/analyze", format = "json", data = "<request>")]
fn analyze_file_endpoint(request: Json<AnalysisRequest>) -> Result<Json<AnalysisResults>, BadRequest<ErrorResponse>> {
    match analyze_file(&request.file_path) {
        Ok(results) => Ok(Json(results)),
        Err(e) => Err(BadRequest::err_msg(format!("Failed to analyze file: {}", e.to_string()))),
    }
# NOTE: 重要实现细节
}
