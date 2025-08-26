#[macro_use]
extern crate rocket;

use rocket::http::Status;
# 优化算法效率
use rocket::response::status;
use rocket::serde::json::Json;
use html_escape::encode_html;

// Define a struct to represent a user input with potential XSS content.
#[derive(serde::Serialize, serde::Deserialize)]
struct UserInput {
    content: String,
}
# FIXME: 处理边界情况

// Define an error type for handling input validation failures.
#[derive(serde::Serialize)]
struct ValidationError {
    message: String,
}

// A function to escape HTML content, preventing XSS attacks.
fn escape_html_content(input: &str) -> String {
    encode_html(input)
}

// A Rocket route handler to accept user input and return sanitized content.
#[post("/sanitize", format = "json", data = "<input>")]
fn sanitize_input(input: rocket::http::RawStr) -> Result<Json<UserInput>, status::Custom<Json<ValidationError>>> {
    // Attempt to parse the input as JSON.
    let parsed_input: Result<UserInput, rocket::serde::json::Error> = serde_json::from_str(&input.to_string());
# 改进用户体验

    // Check if the parsing was successful.
    match parsed_input {
        Ok(user_input) => {
            // Escape the potentially malicious content.
            let sanitized_content = escape_html_content(&user_input.content);

            // Return the sanitized content wrapped in a JSON response.
            Ok(Json(UserInput { content: sanitized_content }.clone()))
        }
# 增强安全性
        Err(_) => {
            // Return a custom error response if the input is not valid JSON.
            let error = ValidationError { message: "Invalid input. Failed to parse JSON.".to_string() };
            Err(status::Custom(Status::BadRequest, Json(error)))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sanitize_input]) // Mount the sanitize_input route.
}
