use rocket::get;
use rocket::Route;
use rocket::response::status;
use rocket::http::Status;
use std::path::Path;
use std::fs;
use zip::ZipArchive;
use std::io::prelude::*;
use std::env;
use rocket::serde::json::Json;

// 定义错误类型
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ErrorResponse {
    error: String,
}

// 主函数
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[get("/unzip/<file>")]
fn unzip(file: String) -> Result<status::Custom<Json<ErrorResponse>>, status::Custom<Json<ErrorResponse>>> {
    // 获取解压路径
    let dest_path = env::temp_dir().join(Path::new(&file).file_stem().unwrap());

    // 创建目标路径
    fs::create_dir_all(&dest_path).unwrap();

    // 打开压缩文件
    let file_path = Path::new(&file);
    let mut zip_file = fs::File::open(&file_path).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorResponse { error: e.to_string() }))
    })?;

    // 解压文件
    let mut archive = ZipArchive::new(&mut zip_file).map_err(|e| {
        status::Custom(Status::InternalServerError, Json(ErrorResponse { error: e.to_string() }))
    })?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            status::Custom(Status::InternalServerError, Json(ErrorResponse { error: e.to_string() }))
        })?;

        let outpath = dest_path.join(file.sanitized_name());
        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| {
                status::Custom(Status::InternalServerError, Json(ErrorResponse { error: e.to_string() }))
            })?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| {
                        status::Custom(Status::InternalServerError, Json(ErrorResponse { error: e.to_string() }))
                    })?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| {
                status::Custom(Status::InternalServerError, Json(ErrorResponse { error: e.to_string() }))
            })?;

            std::io::copy(&mut file, &mut outfile).map_err(|e| {
                status::Custom(Status::InternalServerError, Json(ErrorResponse { error: e.to_string() }))
            })?;
        }
    }

    Ok(status::Custom(Status::Ok, Json(ErrorResponse { error: "".to_string() })))
}