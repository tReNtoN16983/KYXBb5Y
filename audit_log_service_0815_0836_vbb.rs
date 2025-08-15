// audit_log_service.rs

// 引入 Rocket 和其他必要的库
#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::serde::json::Json;
use rocket::State;
use rocket::response::status;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
struct AuditLog {
    // 审计日志数据结构
    timestamp: u64,
    action: String,
    user_id: Option<String>,
    success: bool,
    details: Option<String>,
}

// 全局审计日志存储
#[global]
lazy_static! {
    static ref AUDIT_LOGS: Arc<Mutex<HashMap<String, Vec<AuditLog>>>> = Arc::new(Mutex::new(HashMap::new()));
}

// 示例服务，记录安全审计日志
#[post("/log", format = "json", data = "<log>")]
fn log_audit(log: Json<AuditLog>, audit_logs: &State<Arc<Mutex<HashMap<String, Vec<AuditLog>>>>>) -> status::Accepted<Json<Vec<AuditLog>>> {
    // 获取当前时间戳
    let current_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;

    // 将时间戳添加到日志条目中
    let mut log = log.0;
    log.timestamp = current_timestamp;

    // 获取或创建日志条目列表
    let mut logs = audit_logs.lock().unwrap();
    logs.entry(log.action.clone()).or_insert_with(Vec::new).push(log);

    // 返回接受状态和日志列表
    status::Accepted(Some(logs.get(&log.action).unwrap_or(&vec![]).clone()))
}

// Rocket launch configuration
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![log_audit])
        .manage(AUDIT_LOGS.clone())
}
