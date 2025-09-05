use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::path::Path;
use image::{self, ImageOutputFormat};
use std::io::Cursor;

// 定义请求参数
#[derive(Deserialize)]
pub struct ResizeRequest {
    pub file_path: String,
# NOTE: 重要实现细节
    pub width: u32,
    pub height: u32,
}

// 定义响应结构体
#[derive(Serialize)]
pub struct ResizeResponse {
    pub message: String,
    pub new_file_path: String,
# 增强安全性
}
# NOTE: 重要实现细节

#[rocket::main]
#[launch(config="development")]
async fn main() -> _ {
    rocket::build()
        .mount("/api", routes![resize_image])
# NOTE: 重要实现细节
        .manage("./images") // 启动时加载图片存储路径
# 改进用户体验
}

// 定义路由处理函数
#[post("/resize", data = "<resize_request>")]
async fn resize_image(
    resize_request: Json<ResizeRequest>,
    images_path: &State<String>,
) -> Result<Json<ResizeResponse>, rocket::http::Status> {
    // 验证图片路径存在
    let image_path = Path::new(&images_path).join(&resize_request.file_path);
    if !image_path.exists() {
        return Err(rocket::http::Status::NotFound);
    }
# 增强安全性

    // 读取图片
    let mut image = image::open(&image_path).map_err(|_| rocket::http::Status::InternalServerError)?;

    // 调整图片尺寸
    let resized_image = image.resize(resize_request.width, resize_request.height, image::imageops::FilterType::Nearest);

    // 创建新的文件名
    let new_file_path = format!("{}_resized.{}", resize_request.file_path, image.format().to_lowercase().extension().unwrap());
    let new_image_path = Path::new(&images_path).join(&new_file_path);
# 增强安全性

    // 保存调整后的图片
    let mut output = image::DynamicImage::ImageRgba8(resized_image);
# 添加错误处理
    output.write_to_file(&new_image_path, image::ImageOutputFormat::from_path(&new_image_path).unwrap())
# 扩展功能模块
        .map_err(|_| rocket::http::Status::InternalServerError)?;

    // 返回响应
# 增强安全性
    Ok(Json(ResizeResponse {
        message: "Image resized successfully".to_string(),
        new_file_path,
    }))
}

/// 定义模块
#[cfg(test)]
mod tests {
# 扩展功能模块
    use super::*;

    #[test]
    fn test_resize_image() {
        let resize_request = ResizeRequest {
            file_path: "example.jpg".to_string(),
            width: 800,
            height: 600,
        };
        let images_path = "./images".to_string();

        // 测试 resize_image 函数
        let response = resize_image(Json(resize_request), &State::new(&images_path));
# 扩展功能模块
        assert!(response.is_ok());
    }
}
# NOTE: 重要实现细节
