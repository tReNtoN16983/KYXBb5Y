use rocket::get;
use rocket::State;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use std::sync::Mutex;

// Define the AuditLog struct to represent an audit log entry.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AuditLog {
    id: String,
    timestamp: String,
    event: String,
    user: String,
    details: String,
}

// Define a type alias for the audit log vector wrapped in a Mutex for thread safety.
type AuditLogs = Mutex<Vec<AuditLog>>;

#[get("/logs")]
// Route handler to retrieve audit logs.
async fn get_logs(logs: &State<AuditLogs>) -> Json<Vec<AuditLog>> {
    let logs_vec = logs.lock().unwrap();
    let logs_clone = logs_vec.clone();
    Json(logs_clone)
}

#[get("/log/<id>")]
// Route handler to retrieve a specific audit log by ID.
async fn get_log(id: String, logs: &State<AuditLogs>) -> Option<Json<AuditLog>> {
    let logs_vec = logs.lock().unwrap();
    let log = logs_vec.iter().find(|log| log.id == id);
    log.map(|log| Json(log.clone()))
}

#[post("/log", format = "json", data = "<log>"])
// Route handler to create a new audit log entry.
async fn create_log(log: Json<AuditLog>, logs: &State<AuditLogs>) -> &'static str {
    let mut logs_vec = logs.lock().unwrap();
    logs_vec.push(log.into_inner().clone());
    "Audit log created successfully"
}

#[launch]
// Rocket launchpad to start the web server.
fn rocket() -> _ {
    rocket::build()
        .manage(AuditLogs::new(vec![]))
        .mount("/api", routes![get_logs, get_log, create_log])
}
