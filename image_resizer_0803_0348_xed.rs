use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::path::PathBuf;
use image::io::Reader as ImageReader;
# 添加错误处理
use image::GenericImageView;
use image::DynamicImage;
# FIXME: 处理边界情况
use std::fs;
use std::io::Result as IoResult;

/// 图片尺寸批量调整请求参数
#[derive(Deserialize, Serialize)]
pub struct ResizeRequest {
    /// 目标尺寸宽度
    width: u32,
    /// 目标尺寸高度
# 增强安全性
    height: u32,
    /// 要调整尺寸的图片路径列表
    image_paths: Vec<String>,
# 扩展功能模块
}

/// 图片尺寸批量调整响应
#[derive(Serialize)]
pub struct ResizeResponse {
    /// 调整后的图片路径列表
    resized_paths: Vec<String>,
    /// 错误信息列表
    errors: Vec<String>,
}

#[rocket::main]
async fn main() -> IoResult<()> {
    // 初始化ROCKET应用
    rocket::build()
        .mount("/api", routes![resize_images])
        .launch()
        .await
        .expect("Failed to start application");
}

/// 批量调整图片尺寸的API路由
#[post("/resize", data = "<resize_request>")]
fn resize_images(resize_request: Json<ResizeRequest>,
                 file_server: &State<PathBuf>) -> Json<ResizeResponse> {
# TODO: 优化性能
    let mut resized_paths = Vec::new();
# 扩展功能模块
    let mut errors = Vec::new();
    
    for path in resize_request.image_paths {
# TODO: 优化性能
        let path = file_server.join(path);
        if path.exists() {
            match resize_image(&path, resize_request.width, resize_request.height) {
                Ok(resized_path) => resized_paths.push(resized_path.display().to_string()),
                Err(e) => errors.push(e.to_string()),
# 添加错误处理
            }
        } else {
            errors.push(format!("Image path not found: {}", path.display()));
# 添加错误处理
        }
    }
    
    Json(ResizeResponse {
        resized_paths,
# 增强安全性
        errors,
    })
# 增强安全性
}

/// 调整单张图片尺寸的函数
fn resize_image(path: &PathBuf, width: u32, height: u32) -> Result<PathBuf, image::ImageError> {
    let img = ImageReader::open(path)?.decode()?;

    let resized_img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let output_path = path.with_extension("resized").with_extension(path.extension().to_str().unwrap_or(""));
    
    // 保存调整后的图片
    image::DynamicImage::ImageRgba8(resized_img).save(output_path)?;
    
    Ok(output_path)
}
