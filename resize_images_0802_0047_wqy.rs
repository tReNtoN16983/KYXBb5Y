use rocket::form::Form;
# FIXME: 处理边界情况
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
# 优化算法效率
use std::path::Path;
use image::{open, ImageOutputFormat, DynamicImage};
use std::fs;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use rocket::get;
use rocket::post;
use rocket::http::Status;
use rocket::serde::json::Json;
# NOTE: 重要实现细节
use rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::Data;
# NOTE: 重要实现细节
use rocket::outcome::Outcome::Success;
use std::path::PathBuf;
# NOTE: 重要实现细节
use rocket::response::Responder;
use rocket::response::Response;
use rocket::response::stream::DataStream;
# TODO: 优化性能
use rocket::Data;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Serialize};
use rocket::form::Form;
use rocket::form::FromForm;
use rocket::response::content;
# 改进用户体验
use lazy_static::lazy_static;
use regex::Regex;
# 扩展功能模块
use rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::Data;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Serialize};
use rocket::form::Form;
use rocket::form::FromForm;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::response::stream::DataStream;
use rocket::Data;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

// Define the request form for resizing
#[derive(FromForm)]
pub struct ResizeRequest {
# 优化算法效率
    width: u32,
    height: u32,
}

// Define the image resize response struct
#[derive(Serialize)]
pub struct ResizeResponse {
# 增强安全性
    message: String,
}

// Define the config struct for image resizing
#[derive(Serialize)]
pub struct ResizeConfig {
    width: u32,
    height: u32,
}

// Define the error enum for resizing errors
#[derive(Debug)]
pub enum ResizeError {
    IoError(io::Error),
    ImageError(image::ImageError),
}
# NOTE: 重要实现细节

// Implement the Error trait for ResizeError
impl std::error::Error for ResizeError {}

// Implement the Display trait for ResizeError
# TODO: 优化性能
impl Display for ResizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ResizeError::IoError(ref err) => write!(f, "{}", err),
            ResizeError::ImageError(ref err) => write!(f, "{}", err),
# 扩展功能模块
        }
    }
}

// Define the image resizing service
pub struct ImageResizer;

impl ImageResizer {
    // Resize a single image
# 增强安全性
    pub fn resize_image(&self, input_path: &Path, output_path: &Path, config: &ResizeConfig) -> Result<(), ResizeError> {
        // Open the image
# NOTE: 重要实现细节
        let img = open(input_path).map_err(ResizeError::ImageError)?;
# 扩展功能模块
        
        // Resize the image
        let resized_img = img.resize(config.width, config.height, image::imageops::FilterType::Nearest);
        
        // Save the resized image
        resized_img.save(output_path).map_err(ResizeError::ImageError)
    }
# TODO: 优化性能
    
    // Resize multiple images
    pub fn resize_images(&self, input_dir: &Path, output_dir: &Path, config: &ResizeConfig) -> Result<(), ResizeError> {
        // Read all files in the input directory
        let entries = fs::read_dir(input_dir).map_err(ResizeError::IoError)?;
# 增强安全性
        
        // Iterate over each file entry
        for entry in entries {
            let entry = entry.map_err(ResizeError::IoError)?;
# TODO: 优化性能
            let path = entry.path();
            
            // Check if the file is an image
            if let Some(ext) = path.extension() {
                if ext != "jpg" && ext != "jpeg" && ext != "png" && ext != "gif" && ext != "bmp" && ext != "tiff" && ext != "webp" {
                    continue;
                }
            }
            
            // Construct the output path
            let output_path = output_dir.join(path.file_name().ok_or_else(|| ResizeError::IoError(io::Error::new(io::ErrorKind::NotFound, "File name not found")))?);
            
            // Resize the image
            self.resize_image(&path, &output_path, config).map_err(ResizeError::ImageError)?;
        }
        
        Ok(())
    }
# TODO: 优化性能
}

// Define the Rocket route for resizing images
#[post("/resize", data = "<resize_request>")]
fn resize_images_route(resizer: ImageResizer, resize_request: Form<ResizeRequest>, input_dir: String, output_dir: String) -> Result<Json<ResizeResponse>, status::InternalServerError<Json<ResizeResponse>>> {
    // Parse the input and output directories
    let input_dir = Path::new(&input_dir);
    let output_dir = Path::new(&output_dir);
    
    // Create the resize configuration
    let config = ResizeConfig {
        width: resize_request.width,
        height: resize_request.height,
    };
    
    // Resize the images
    let result = resizer.resize_images(input_dir, output_dir, &config);
    
    // Handle the result
    match result {
        Ok(_) => Ok(Json(ResizeResponse { message: "Images resized successfully".to_string() })),
        Err(e) => Err(status::InternalServerError(Some(Json(ResizeResponse { message: format!("Error resizing images: {}", e) }) ))),
    }
}

// Define the main function
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![resize_images_route])
}
