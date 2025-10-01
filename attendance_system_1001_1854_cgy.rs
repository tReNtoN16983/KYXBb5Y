// attendance_system.rs
// 这是一个使用RUST和ROCKET框架实现的考勤打卡系统。

#[macro_use]
extern crate rocket;

// 引入Rocket框架的其他组件
use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义员工结构体
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Employee {
    id: u32,
    name: String,
}

// 定义考勤记录结构体
#[derive(Debug, Serialize, Deserialize)]
struct AttendanceRecord {
    employee_id: u32,
    timestamp: String,
}

// 定义考勤管理系统状态
struct AttendanceSystem {
    records: Mutex<HashMap<u32, Vec<AttendanceRecord>>>,
}

// 实现AttendanceSystem结构体
impl AttendanceSystem {
    // 初始化新的考勤系统
    fn new() -> Self {
        AttendanceSystem {
            records: Mutex::new(HashMap::new()),
        }
    }

    // 添加考勤记录
    fn add_attendance(&self, employee_id: u32, timestamp: String) -> Result<(), String> {
        let mut records = self.records.lock().unwrap();
        records.entry(employee_id).or_insert_with(Vec::new).push(AttendanceRecord {
            employee_id,
            timestamp,
        });
        Ok(())
    }

    // 获取员工的考勤记录
    fn get_attendance_records(&self, employee_id: u32) -> Result<Vec<AttendanceRecord>, String> {
        let records = self.records.lock().unwrap();
        records.get(&employee_id).cloned().ok_or_else(|| "Employee not found.".to_string())
    }
}

// 启动Rocket应用程序
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            attend,
            get_attendance,
        ])
        .manage(AttendanceSystem::new())
}

// 处理考勤打卡请求
#[post("/attend", format = "json", data = "<employee_id>")]
async fn attend(employee_id: Json<u32>, system: &State<AttendanceSystem>) -> Result<&'static str, Status> {
    let timestamp =chrono::Utc::now().to_rfc3339();
    match system.add_attendance(employee_id.0, timestamp) {
        Ok(_) => Ok("Attendance recorded successfully."),
        Err(e) => Err(Status::InternalServerError),
    }
}

// 获取员工考勤记录
#[get("/attendance/<employee_id>")]
fn get_attendance(employee_id: u32, system: &State<AttendanceSystem>) -> Result<Json<Vec<AttendanceRecord>>, Status> {
    match system.get_attendance_records(employee_id) {
        Ok(records) => Ok(Json(records)),
        Err(_) => Err(Status::NotFound),
    }
}
