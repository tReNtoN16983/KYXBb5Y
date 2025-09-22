 * error handling, and following Rust best practices for maintainability and scalability.
 */

use rocket::get;
use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::{Serialize};
use rocket::State;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[macro_use]
extern crate rocket;

// Define a structure to represent an audit log entry.
#[derive(Serialize)]
struct AuditLogEntry {
    timestamp: u64,
    action: String,
    details: String,
}

// Define a structure to hold the audit log service state.
struct AuditLogService {
    // A mutex-guarded hash map to store audit log entries.
    entries: Mutex<HashMap<String, Vec<AuditLogEntry>>>,
}

// Implement methods for the AuditLogService structure.
impl AuditLogService {
    // Creates a new AuditLogService.
    fn new() -> Self {
        AuditLogService {
            entries: Mutex::new(HashMap::new()),
        }
    }

    // Logs an audit entry.
    fn log(&self, action: &str, details: &str) {
        let mut entries = self.entries.lock().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

        let entry = AuditLogEntry {
            timestamp: now,
            action: action.to_string(),
            details: details.to_string(),
        };

        // Ensure the key exists in the hash map.
        entries.entry(action.to_string()).or_insert_with(Vec::new).push(entry);
    }

    // Retrieves all audit log entries for a given action.
    fn get_audit_logs(&self, action: &str) -> Vec<AuditLogEntry> {
        let entries = self.entries.lock().unwrap();
        entries.get(action).cloned().unwrap_or_default()
    }
}

// Define a route to get audit logs for a specific action.
#[get("/audit/<action>")]
fn get_audit_logs(action: String, service: &State<AuditLogService>) -> Json<Vec<AuditLogEntry>> {
    service.get_audit_logs(&action)
}

// Define a route to log an audit entry.
#[post("/audit/<action>", format = "json", data = "<details>")]
fn log_audit(action: String, details: Json<String>, service: &State<AuditLogService>) {
    service.log(&action, &details.0);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_audit_logs, log_audit])
        .manage(AuditLogService::new())
}
