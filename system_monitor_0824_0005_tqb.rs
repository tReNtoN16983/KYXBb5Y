use rocket::get;
use rocket::Route;
use std::collections::HashMap;
use sysinfo::{System, SystemExt};

// Define the SystemInfo struct that will hold our system information.
#[derive(Debug, Clone)]
struct SystemInfo {
    pub info: System,
}

// Define the routes for the Rocket application.
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

// Define a custom error type for our application.
#[derive(Debug, serde::Serialize)]
enum SystemMonitorError {
    FailedToCollectData,
}

// Implement a Fairing to handle error serialization for our custom errors.
#[rocket::fairing]
fn handle_errors<'r>(
    rocket::State<SystemInfo>: &'r SystemInfo,
) -> rocket::fairing::AdHoc<'r> {
    rocket::fairing::AdHoc::on_error(
        |request, err| {
            if let Some(custom_err) = err.downcast_ref::<SystemMonitorError>() {
                rocket::response::status::Status::InternalServerError(Some(rocket::serde::json::Json::from(custom_err).into_value()))
            } else {
                Err(err)
            }
        },
    )
}

// Define a route to get system information.
#[get("/sysinfo")]
fn get_system_info(system_info: rocket::State<SystemInfo>) -> Result<HashMap<String, String>, SystemMonitorError> {
    let info = &system_info.info;
    if info.refresh_all() {
        Ok(
            HashMap::from(
                vec![
                    ("hostname".to_string(), info.name().unwrap_or_else(|| "Unknown".to_string())),
                    ("cpu_usage".to_string(), format!("{}", info.cpus().iter().map(|cpu| cpu.cpu_usage()).fold(0, |acc, x| acc + x) as f32 / info.cpus().len() as f32)),
                    ("ram_total".to_string(), format!("{}", info.total_memory().unwrap_or_default())),
                    ("ram_used".to_string(), format!("{}", info.used_memory().unwrap_or_default())),
                ]
            ),
        )
    } else {
        Err(SystemMonitorError::FailedToCollectData)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(SystemInfo { info: System::new_all().expect("Failed to initialize system info") })
        .mount("/", routes![get_system_info])
        .attach(handle_errors)
}
