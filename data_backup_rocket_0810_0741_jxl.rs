use rocket::get;
use rocket::post;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status;
use rocket::response::Status;
use rocket::http::Status as HttpStatus;
use std::fs::File;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;
use std::env;

#[macro_use]
extern crate serde_json;

// 定义请求和响应的数据结构
#[derive(Serialize, Deserialize)]
struct BackupRequest {
    file_path: String,
}

#[derive(Serialize, Deserialize)]
struct RestoreRequest {
    backup_file: String,
}

// 创建一个错误类型，用于处理备份和恢复操作中的错误
#[derive(Debug)]
enum BackupRestoreError {
    FileNotFound,
    IoError(Error),
}

// 实现错误类型的错误处理
impl std::fmt::Display for BackupRestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupRestoreError::FileNotFound => write!(f, "File not found"),
            BackupRestoreError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for BackupRestoreError {}

// 备份文件的函数
#[get("/backup")]
fn backup(file_path: String) -> Result<Json<String>, status::Custom<&'static str>> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(status::Custom(HttpStatus::NotFound, "File not found"));
    }

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => return Err(status::Custom(HttpStatus::InternalServerError, e.to_string())),
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(status::Custom(HttpStatus::InternalServerError, e.to_string()));
    }

    // 将文件内容写入备份文件
    let backup_path = format!("./backups/{}", file_path);
    let mut backup_file = match File::create(&backup_path) {
        Ok(file) => file,
        Err(e) => return Err(status::Custom(HttpStatus::InternalServerError, e.to_string())),
    };
    if let Err(e) = backup_file.write_all(contents.as_bytes()) {
        return Err(status::Custom(HttpStatus::InternalServerError, e.to_string()));
    }

    Ok(Json(json!("Backup successful")))
}

// 恢复文件的函数
#[post("/restore")]
fn restore(backup_file: Json<RestoreRequest>) -> Result<Json<String>, status::Custom<&'static str>> {
    let backup_path = Path::new(&backup_file.backup_file);
    if !backup_path.exists() {
        return Err(status::Custom(HttpStatus::NotFound, "Backup file not found"));
    }

    let mut file = match File::open(&backup_path) {
        Ok(file) => file,
        Err(e) => return Err(status::Custom(HttpStatus::InternalServerError, e.to_string())),
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(status::Custom(HttpStatus::InternalServerError, e.to_string()));
    }

    // 将备份文件内容恢复到原文件
    let original_path = backup_path.file_stem().unwrap().to_str().unwrap().to_string();
    let mut original_file = match File::create(&original_path) {
        Ok(file) => file,
        Err(e) => return Err(status::Custom(HttpStatus::InternalServerError, e.to_string())),
    };
    if let Err(e) = original_file.write_all(contents.as_bytes()) {
        return Err(status::Custom(HttpStatus::InternalServerError, e.to_string()));
    }

    Ok(Json(json!("Restore successful")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![backup, restore])
}
