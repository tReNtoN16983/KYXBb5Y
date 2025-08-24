use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::State;
use std::path::PathBuf;
use image::{DynamicImage, GenericImageView, ImageError, ImageResult};
use image::imageops::resize;
use rocket::fs::TempFile;
use rocket::response::stream_to_producer;
use rocket::response::Response;
use rocket::ResponseBuilder;
use rocket::http::Status;
use std::io::Cursor;
use rocket::serde::json::Json;
use rocket::Config;
use rocket::Rocket;

// 定义请求参数
#[derive(Deserialize, Debug)]
pub struct ResizeParams {
    pub width: u32,
    pub height: u32,
}

// 定义响应数据结构
#[derive(Serialize, Debug)]
pub struct ResizeResponse {
    pub status: String,
    pub message: String,
}

#[macro_use]
extern crate rocket;

// 定义主程序结构体
struct ImageResizer {
    temp_dir: PathBuf,
}

#[rocket::main]
async fn main() {
    let mut config = Config::development().address("127.0.0.1").port(8000);
    let temp_dir = PathBuf::from("/tmp/image_resizer");
    rocket::custom(config)
        .manage(ImageResizer { temp_dir })
        .mount("/", routes![resize_images])
        .launch()
        .await;
}

// 定义路由
#[post("/resize")]
async fn resize_images(
    form: Form<ResizeParams>,
    file: TempFile,
    client: rocket::request::Client<'_>,
    resize_service: &State<ImageResizer>,
) -> Result<Json<ResizeResponse>, Status> {
    // 读取文件并转换为DynamicImage
    let img = image::open(file.path()).map_err(|e| {
        eprintln!("Error opening image: {}", e);
        Status::InternalServerError
    })?;

    // 根据参数调整图片尺寸
    let resized_img = resize(&img, form.width, form.height, image::imageops::FilterType::Nearest);

    // 保存调整后的图片到临时目录
    let output_path = resize_service.temp_dir.join("resized_image.png");
    resized_img.save(output_path.clone()).map_err(|e| {
        eprintln!("Error saving image: {}", e);
        Status::InternalServerError
    })?;

    // 返回成功响应
    Ok(Json(ResizeResponse {
        status: "success".to_string(),
        message: format!("Image resized and saved to {}", output_path.to_str().unwrap()),
    }))
}
