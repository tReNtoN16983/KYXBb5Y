use rocket::get;
use rocket::Route;
use rocket_contrib::json::Json;
use std::process::{Command, Child, ExitStatus};
use std::io::{self, Error, ErrorKind};
use std::time::Duration;
use rocket::response::status::BadRequest;
use rocket::response::status::InternalServerError;
use rocket::response::status::Ok;

#[macro_use]
extern crate rocket;

// 定义一个结构体来存储进程信息
#[derive(Debug, Serialize, Deserialize)]
struct ProcessInfo {
    process_id: u32,
    command: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ProcessStatus {
    Running,
    Stopped,
    Error(String),
}

// 定义一个处理进程操作的结构体
#[rocket::main]
struct ProcessManager;

impl ProcessManager {
    // 启动一个新进程
    #[get("/start_process?<command>&<timeout>")]
    fn start_process(command: String, timeout: Option<u32>) -> Result<Json<ProcessInfo>, BadRequest<String>> {
        let mut child = match Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn() {
            Ok(process) => process,
            Err(_) => return Err(BadRequest(Some("Failed to start process".to_string()))),
        };

        let mut status = match child.try_wait() {
            Ok(Some(status)) => ProcessStatus::Stopped,
            Ok(None) => ProcessStatus::Running,
            Err(_) => ProcessStatus::Error("Failed to wait for process".to_string()),
        };

        if let Some(to) = timeout {
            // 等待进程结束或超时
            if let Err(_) = child.wait_timeout(Duration::from_secs(to)) {
                status = ProcessStatus::Error("Process timed out".to_string());
            } else {
                status = ProcessStatus::Stopped;
            }
        }

        match status {
            ProcessStatus::Running => Ok(Json(ProcessInfo {
                process_id: child.id() as u32,
                command,
                status: "Running".to_string(),
            })),
            ProcessStatus::Stopped => Ok(Json(ProcessInfo {
                process_id: child.id() as u32,
                command,
                status: "Stopped".to_string(),
            })),
            ProcessStatus::Error(ref err) => Err(BadRequest(Some(err.clone()))),
        }
    }

    // 停止一个进程
    #[get("/stop_process?<process_id>")]
    fn stop_process(process_id: u32) -> Result<Json<ProcessInfo>, InternalServerError<String>> {
        match nix::unistd::kill(nix::unistd::Pid::from_raw(process_id as i32), nix::sys::signal::SIGKILL) {
            Ok(_) => Ok(Json(ProcessInfo {
                process_id,
                command: "kill".to_string(),
                status: "Stopped".to_string(),
            })),
            Err(_) => Err(InternalServerError(Some("Failed to stop process".to_string()))),
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/process", routes![ProcessManager::start_process, ProcessManager::stop_process])
}
