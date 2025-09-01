use rocket::get;
use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use std::process::Command;
use std::io::{self, Error};
use std::collections::HashMap;

// 定义一个结构体来存储进程信息
#[derive(Serialize, Deserialize, Debug)]
struct Process {
    command: String,
    pid: u32,
    status: String,
}

// 定义一个结构体来存储进程管理器的状态
struct ProcessManager {
    processes: HashMap<u32, Process>,
}

impl ProcessManager {
    // 创建一个新的进程管理器
    pub fn new() -> Self {
        ProcessManager {
            processes: HashMap::new(),
        }
    }

    // 添加一个新的进程到管理器
    pub fn add_process(&mut self, command: &str) -> Result<u32, Error> {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(command)
            .output()?;

        let pid = output.status.code().unwrap_or(0) as u32;
        let process = Process {
            command: command.to_string(),
            pid,
            status: "Running".to_string(),
        };

        self.processes.insert(pid, process);
        Ok(pid)
    }

    // 停止一个进程
    pub fn stop_process(&mut self, pid: u32) -> Result<(), Error> {
        if let Some(process) = self.processes.remove(&pid) {
            let _ = Command::new("/bin/sh")
                .arg("-c")
                .arg(&format!("kill {}", pid))
                .output()?;

            Ok(())
        } else {
            Err(Error::new(io::ErrorKind::NotFound, "Process not found"))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/process", routes![add_process, list_processes, stop_process])
}

// 定义API路由
#[get("/add")]
fn add_process(command: String) -> Result<Json<Process>, Error> {
    let mut manager = ProcessManager::new();
    let pid = manager.add_process(&command)?;
    let process = manager.processes.get(&pid).unwrap().clone();
    Ok(Json(process))
}

// 列出所有进程
#[get("/list")]
fn list_processes(manager: rocket::State<ProcessManager>) -> Result<Json<Vec<Process>>, Error> {
    let processes: Vec<Process> = manager.processes.values().cloned().collect();
    Ok(Json(processes))
}

// 停止一个进程
#[get("/stop/<pid>")]
fn stop_process(pid: u32, manager: rocket::State<ProcessManager>) -> Result<(), Error> {
    manager.stop_process(pid)?;
    Ok(())
}
