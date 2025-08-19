// system_monitor.rs
// 这是一个基于RUST和ROCKET框架的系统性能监控工具。
// 它提供了获取系统资源信息的接口。

use rocket::get;
use rocket::serde::json::Json;
use std::process::Command;
use std::str;
use serde::Serialize;

// 定义一个结构体来存储系统信息
#[derive(Serialize)]
struct SystemInfo {
    pub uptime: String,
    pub memory_usage: String,
    pub cpu_usage: String,
}

// 实现一个服务来获取系统信息
#[get("/system_info")]
// 这个端点返回系统的基本信息
fn get_system_info() -> Result<Json<SystemInfo>, &'static str> {
    let uptime = get_uptime();
    let memory_usage = get_memory_usage();
    let cpu_usage = get_cpu_usage();

    if uptime.is_err() || memory_usage.is_err() || cpu_usage.is_err() {
        return Err("Failed to fetch system information");
    }

    Ok(Json(SystemInfo {
        uptime: uptime.unwrap(),
        memory_usage: memory_usage.unwrap(),
        cpu_usage: cpu_usage.unwrap(),
    }))
}

// 获取系统运行时间
fn get_uptime() -> Result<String, &'static str> {
    let output = Command::new("uptime"). Output();
    let output = output.output().map_err(|_| "Failed to execute uptime command")?;
    let output = str::from_utf8(&output.stdout).map_err(|_| "Failed to parse uptime output")?;
    Ok(output.to_string())
}

// 获取内存使用情况
fn get_memory_usage() -> Result<String, &'static str> {
    let output = Command::new("free").arg("-m"). Output();
    let output = output.output().map_err(|_| "Failed to execute free command")?;
    let output = str::from_utf8(&output.stdout).map_err(|_| "Failed to parse free output")?;
    Ok(output.to_string())
}

// 获取CPU使用情况
fn get_cpu_usage() -> Result<String, &'static str> {
    let output = Command::new("top").arg("-b").arg("-n").arg("1"). Output();
    let output = output.output().map_err(|_| "Failed to execute top command")?;
    let output = str::from_utf8(&output.stdout).map_err(|_| "Failed to parse top output")?;
    Ok(output.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_system_info])
}
