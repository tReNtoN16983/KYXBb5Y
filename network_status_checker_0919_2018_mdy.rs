use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize};
use std::net::TcpStream;
use std::io::Result;
use anyhow::Result as AnyResult;
use std::time::Duration;
use rocket::fairing::AdHoc;
use rocket::Rocket;

// Define a struct to represent network connection status
#[derive(Serialize)]
pub struct NetworkStatus {
    host: String,
    port: u16,
    connected: bool,
    error_message: Option<String>,
}

// Define a configuration structure with default values
#[derive(Debug, Default)]
struct Config {
    timeout: Duration,
}

// Define a fairing that extends Rocket with configuration
#[rocket::fairing]
pub struct NetworkCheckConfig {
    config: Config,
}

impl NetworkCheckConfig {
    pub fn new() -> Self {
        NetworkCheckConfig {
            config: Config { timeout: Duration::from_secs(5) },
        }
    }
}

#[rocket::launch]
fn rocket() -> Rocket<rocket::Build> {
    rocket::build()
        .attach(NetworkCheckConfig::new())
        .mount("/", routes![check_network_status])
}

// The route handler to check network connection status
#[get("/check/<host>/<port>?<timeout>")]
async fn check_network_status(host: String, port: u16, timeout: Option<u64>) -> AnyResult<Json<NetworkStatus>, status::InternalServerError<&'static str>> {
    let config = rocket::fairing::get::<NetworkCheckConfig>().unwrap().config;
    let timeout = timeout.map(Duration::from_secs).unwrap_or(config.timeout);

    // Attempt to connect to the specified host and port with a timeout
    let connected = match TcpStream::connect_timeout((host.as_str(), port), timeout) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            false
        },
    };

    // Return the connection status as a JSON response
    Ok(Json(NetworkStatus {
        host,
        port,
        connected,
        error_message: if connected { None } else { Some(e.to_string()) },
    }))
}

// Add any necessary documentation or comments here

/*
This Rust application uses the Rocket framework to create a network connection status checker.
It allows users to check the connection status for a given host and port with an optional timeout.
The application returns a JSON response containing the connection status and any error messages.

Usage example:
GET /check/google.com/80?timeout=10
*/
