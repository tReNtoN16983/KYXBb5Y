 * Features:
 * - A Rocket server with a single route for performance testing.
 * - Use of `reqwest` for making HTTP requests to the route.
 * - Use of `tokio` for async runtime.
 */

use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[macro_use]
extern crate rocket;

// Define a struct for the performance test results.
#[derive(Serialize)]
struct TestResult {
    /// The number of requests sent.
    pub requests: u32,
    /// The total time taken for the requests in milliseconds.
    pub total_time_ms: u128,
    /// The average time taken per request in milliseconds.
    pub average_time_ms: f64,
}

#[get("/test")]
/// A simple route to test performance.
/// This route will return a JSON with a message indicating it is alive and functioning.
fn test_route() -> Json<String> {
    Json("Performance Test Route is Up and Running.".to_string())
}

#[tokio::main]
/// The main function performs a series of HTTP GET requests to the `/test` route.
/// It calculates and prints the performance metrics of these requests.
async fn main() {
    let client = reqwest::Client::new();
    let url = "http://localhost:8000/test";
    let requests: u32 = 100; // Number of requests to send.
    let mut total_time: u128 = 0;
    let mut start = Instant::now();

    for _ in 0..requests {
        let start_request = Instant::now();

        match client.get(url).send().await {
            Ok(response) => {
                if response.status() == Status::Ok {
                    let duration = start_request.elapsed();
                    total_time += duration.as_millis() as u128;
                } else {
                    eprintln!("Error: Received non-200 status code.");
                }
            },
            Err(e) => {
                eprintln!("Error: Failed to send request. Error: {:?}", e);
            },
        }

        sleep(Duration::from_millis(10)).await; // Throttle requests to avoid overwhelming the server.
    }

    let end = start.elapsed();
    let average_time_ms = (end.as_secs_f64() * 1000.0) / requests as f64;

    println!("Completed {} requests.", requests);
    println!("Total time taken: {} ms", end.as_millis());
    println!("Average time per request: {:.2} ms", average_time_ms);

    let test_result = TestResult {
        requests,
        total_time_ms: total_time,
        average_time_ms,
    };

    // Serialize the test results to JSON and print them.
    println!("Test Results: {:?}", serde_json::to_string(&test_result).unwrap());
}
