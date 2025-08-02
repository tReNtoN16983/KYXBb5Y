#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;
use std::fmt;
# 改进用户体验

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket\_serde")]
struct ApiResponse {
# NOTE: 重要实现细节
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
# NOTE: 重要实现细节
    #[serde(skip_serializing_if = "Vec::is_empty")]
    data: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}
# 扩展功能模块

#[derive(Serialize)]
#[serde(crate = "rocket\_serde")]
struct ApiErrorResponse {
    error: String,
}

// 自定义状态码枚举
#[derive(Debug)]
enum ApiResponseStatus {
    Success,
    Error,
}
# 改进用户体验

// 格式化API响应的工具函数
fn format_response<T: Serialize>(data: T, status: ApiResponseStatus) -> Json<ApiResponse> {
    let mut response = ApiResponse {
        status: Some(match status {
# TODO: 优化性能
            ApiResponseStatus::Success => "success".to_string(),
# 优化算法效率
            ApiResponseStatus::Error => "error".to_string(),
        }),
        data: Some(vec![format!("{:?}", data)]),
        message: None,
        error: None,
# NOTE: 重要实现细节
    };
    if status == ApiResponseStatus::Error {
        response.error = Some("An error occurred".to_string());
    }
    Json(response)
}

// routes! 宏定义路由
#[macro_export]
macro_rules! routes {
    ($($path:expr => $handler:ident),*) => {
        use rocket::routes!;
        $(
            routes![$path => $handler]
        )*
    }
}

// 定义API响应格式化工具的服务
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            "/response" => handle_response,
            "/error" => handle_error,
        ])
}
# NOTE: 重要实现细节

// 处理成功的API响应
#[get("/response")]
fn handle_response() -> Json<ApiResponse> {
# 添加错误处理
    format_response("Hello, world!", ApiResponseStatus::Success)
}

// 处理错误的API响应
#[get("/error")]
fn handle_error() -> status::Custom<Json<ApiErrorResponse>> {
    let error = ApiErrorResponse {
# 增强安全性
        error: "Something went wrong".to_string(),
# 优化算法效率
    };
    (status::Status::InternalServerError, Json(error))
}
