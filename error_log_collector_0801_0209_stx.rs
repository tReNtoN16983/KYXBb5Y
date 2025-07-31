 * It provides an endpoint to receive error logs and stores them in memory.
 * For simplicity, error logs are stored in a Vec<String> but could be extended to
 * write logs to a file or database in the future.
 */

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use std::sync::Mutex;
use std::collections::VecDeque;

// Define a structure to hold the error logs
struct ErrorLogs {
    logs: Mutex<VecDeque<String>>,
}

// Define a request structure for the error log payload
#[derive(serde::Deserialize, Debug)]
struct ErrorLogPayload {
    message: String,
    level: String,
    timestamp: String, // A simple string representation of the timestamp
}

#[rocket::main]
async fn main() {
    // Initialize the error logs vector
    let error_logs = ErrorLogs {
        logs: Mutex::new(VecDeque::new()),
    };

    // Launch the Rocket server
    rocket::build()
        .mount("/logs", routes![
            log_error,
            get_logs,
        ])
        .manage(error_logs)
        .launch()
        .await
        .expect("Rocket server failed to launch.");
}

// Endpoint to log an error
#[post("/error", format = "json", data = "<log>")]
fn log_error(log: Json<ErrorLogPayload>, error_logs: &State<ErrorLogs>) -> &'static str {
    // Push the error log to the vector
    let mut logs = error_logs.logs.lock().unwrap();
    logs.push_back(format!(
        "Timestamp: {}, Level: {}, Message: {}", log.timestamp, log.level, log.message
    ));

    "Error logged successfully."
}

// Endpoint to retrieve error logs
#[get("/logs")]
fn get_logs(error_logs: &State<ErrorLogs>) -> Json<Vec<String>> {
    // Clone the logs vector to return it
    let logs = error_logs.logs.lock().unwrap().clone();
    Json(logs.into_iter().collect())
}
