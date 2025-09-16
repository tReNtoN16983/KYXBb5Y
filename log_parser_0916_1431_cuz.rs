use rocket::serde::json::Json;
use rocket::State;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;

/// 日志记录器，用于解析日志文件
struct LogParser {
    /// 存储解析后的日志数据
    logs: HashMap<String, u32>,
}

impl LogParser {
    /// 创建一个新的日志解析器
    pub fn new() -> Self {
        LogParser {
            logs: HashMap::new(),
        }
    }

    /// 解析给定路径的日志文件
    pub fn parse_log_file<P: AsRef<Path> + ?Sized>(&mut self, path: &P) -> Result<(), Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.is_empty() {
                continue;
            }

            // 假设日志文件的每一行都以一个标识符开头，我们将其作为键
            let key = parts[0].to_string();
            self.logs.entry(key).or_insert(0);
            self.logs.get_mut(&key).unwrap() += 1;
        }

        Ok(())
    }
}

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/log", routes![parse_log])
        .manage(LogParser::new())
}

/// 路由处理函数，解析日志文件并返回解析结果
#[get("/parse_log?<file_path>&<file_path>")]
fn parse_log(file_path: String, log_parser: &State<LogParser>) -> Json<Vec<String>> {
    match log_parser.parse_log_file(file_path.as_str()) {
        Ok(_) => {
            let mut results: Vec<String> = Vec::new();
            for (key, value) in &log_parser.logs {
                results.push(format!("{}": {}", key, value));
            }
            Json(results)
        },
        Err(e) => {
            Json(vec![e.to_string()])
        },
    }
}
