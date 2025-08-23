// log_parser.rs
//
// 一个简单的日志文件解析工具，使用Rust和Rocket框架。

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

// 定义一个日志条目结构体
#[derive(Debug, Serialize, Deserialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .manage(read_log_file) // 将日志文件内容管理起来
}

// 读取日志文件的函数
fn read_log_file<'a>() -> io::Result<Vec<LogEntry>> {
    let path = Path::new("./log.txt");
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut log_entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // 假设日志格式为："2023-04-01 12:00:00 INFO This is a log message"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue; // 跳过格式不正确的行
        }
        let timestamp = parts[0].to_string();
        let level = parts[1].to_string();
        let message = parts[2..].join(" ");
        log_entries.push(LogEntry { timestamp, level, message });
    }

    Ok(log_entries)
}

// 根路由，返回日志文件解析结果
#[get("/")]
fn index(log_entries: &State<Vec<LogEntry>>) -> Json<Vec<LogEntry>> {
    Json(log_entries.clone())
}
