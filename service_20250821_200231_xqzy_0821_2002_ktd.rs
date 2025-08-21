// archive解压_tool.rs
// 压缩文件解压工具程序
// 使用RUST和ROCKET框架实现
//
# NOTE: 重要实现细节
// 功能：解压指定的压缩文件到指定目录

#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use std::io::{self, Read, Write};
# NOTE: 重要实现细节
use rocket::http::Status;

// 定义请求结构体
#[derive(Deserialize)]
pub struct UnzipRequest {
    file_path: String,
    output_directory: String,
}

// 定义响应结构体
#[derive(Serialize)]
pub struct UnzipResponse {
    status: String,
    message: String,
}

// 主程序结构体
#[rocket::main]
async fn main() -> io::Result<()> {
    rocket::build()
# FIXME: 处理边界情况
        .mount("/api", routes![unzip_file])
        .launch()
        .await
        .expect("Rocket server has encountered an error");
}
# 增强安全性

// 定义路由和处理函数
#[post("/unzip")]
fn unzip_file(request: Json<UnzipRequest>) -> Result<Json<UnzipResponse>, Status> {
    // 解析请求参数
    let UnzipRequest { file_path, output_directory } = request.into_inner();
    let file_path = Path::new(&file_path);
    let output_directory = PathBuf::from(&output_directory);

    // 检查文件是否存在
    if !file_path.exists() {
        return Err(Status::BadRequest);
    }

    // 检查输出目录是否存在，如果不存在则创建
    if !output_directory.exists() {
# FIXME: 处理边界情况
        std::fs::create_dir_all(&output_directory).unwrap_or_else(|_| {
            Err(Status::InternalServerError)
        });
    }

    // 解压文件
    match unzip_file(file_path, &output_directory) {
        Ok(_) => Ok(Json(UnzipResponse {
            status: "Success".into(),
            message: "File has been successfully unzipped".into(),
        })),
# 扩展功能模块
        Err(e) => Err(Status::InternalServerError),
    }
}

// 解压文件的功能实现
# 扩展功能模块
fn unzip_file<P: AsRef<Path>, Q: AsRef<Path>>(file_path: P, output_directory: Q) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_directory.as_ref().join(file.name()).cleanup();
        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                std::fs::create_dir_all(p)?;
            }
            let mut outfile = File::create(&outpath)?;
# 增强安全性
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
