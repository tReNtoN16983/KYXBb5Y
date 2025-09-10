use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::response::status;
# 添加错误处理
use rocket::State;
# 扩展功能模块
use rocket::Config;
use image::{DynamicImage, GenericImageView, ImageError};
use std::path::Path;
use std::fs;
use std::io::Write;
use std::io::ErrorKind;
use rocket::response::Responder;
use rocket::serde::json::serde_json::json;
# FIXME: 处理边界情况

// 定义一个结构体来接收来自前端的请求数据
#[derive(FromForm)]
#[form(multipart = true)]
# TODO: 优化性能
struct ImageResizeRequest {
    files: Vec<tempfile::NamedTempFile>,
# TODO: 优化性能
    width: u32,
    height: u32,
    output_dir: String,
}

// 定义一个响应类型来返回操作结果
#[derive(Serialize, Deserialize)]
struct ImageResizeResponse {
    success: bool,
    message: String,
}

// 实现 Responder trait 以便可以直接返回 JSON 响应
impl<'r> Responder<'r, 'static> for ImageResizeResponse {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let body = serde_json::to_string(&self).unwrap();
        Ok(rocket::response::Response::build()
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.len(), rocket::response::body::BoxBody::from(body))
# FIXME: 处理边界情况
            .ok())
    }
}

#[macro_use] extern crate rocket;

// 火箭应用入口
#[launch]
fn rocket(config: Config) -> _ {
# TODO: 优化性能
    rocket::build()
        .manage(config)
        .mount("/image_resize", routes![resize_images])
}

// 图片尺寸调整的路由
# 添加错误处理
#[post("/resize", format = "json", data = "<form>")]
async fn resize_images(form: Json<ImageResizeRequest>) -> ImageResizeResponse {
    let ImageResizeRequest { files, width, height, output_dir } = form.into_inner();
    let mut success = true;
    let mut message = String::new();

    for file in files {
# 优化算法效率
        let path = file.path().to_str().unwrap().to_string();
        match image::open(&path) {
            Ok(image) => {
# TODO: 优化性能
                let resized_image = image.resize(width, height, image::imageops::FilterType::Nearest);
                let output_path = Path::new(&output_dir).join(Path::new(&path).file_name().unwrap());

                match resized_image.save(&output_path) {
                    Ok(_) => {
                        // 记录成功的消息
                        message.push_str(&format!("Successfully resized image at {}
", output_path.display()));
                    },
                    Err(e) => {
                        // 记录错误的消息
                        message.push_str(&format!("Failed to save resized image at {}: {}
# 添加错误处理
", output_path.display(), e));
# 扩展功能模块
                        success = false;
                    },
                }
            },
            Err(e) => {
                // 记录错误的消息
# FIXME: 处理边界情况
                message.push_str(&format!("Failed to open image at {}: {}
", path, e));
                success = false;
            },
        }
    }

    ImageResizeResponse {
        success,
        message,
    }
}

// 确保输出目录存在
fn ensure_output_dir_exists(output_dir: &str) -> std::io::Result<()> {
    let dir_path = Path::new(output_dir);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }
    Ok(())
}

// 错误处理函数
fn handle_error(e: ImageError) -> status::Custom<Json<ImageResizeResponse>> {
# NOTE: 重要实现细节
    let mut response = ImageResizeResponse {
        success: false,
        message: String::new(),
    };

    match e {
        ImageError::IoError(ref io_err) if io_err.kind() == ErrorKind::NotFound => {
            // 处理文件找不到的情况
            response.message = "The specified file was not found.".to_string();
        },
        _ => {
# NOTE: 重要实现细节
            // 处理其他错误情况
            response.message = format!("An error occurred: {}", e);
# 添加错误处理
        },
    }

    status::BadRequest(Some(Json(response)))
}
