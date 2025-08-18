use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

// 定义错误日志的结构体
#[derive(Debug, Serialize, Deserialize)]
struct ErrorLog {
    timestamp: String,
    message: String,
}

// 定义错误日志收集器
#[derive(Debug, Clone)]
struct ErrorLogger {
    log_file: String,
}

// 实现ErrorLogger结构体的方法
impl ErrorLogger {
    // 创建新的ErrorLogger实例
    fn new(log_file: String) -> Self {
        ErrorLogger { log_file }
    }

    // 将错误日志写入文件
    fn log_error(&self, error: ErrorLog) -> io::Result<()> {
        let mut file = File::options()
            .append(true)
            .open(&self.log_file)?;

        writeln!(file, "{}", serde_json::to_string(&error)?)?;

        Ok(())
    }
}

#[post(