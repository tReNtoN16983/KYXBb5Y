use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::net::TcpStream;
use std::str::FromStr;
use rocket::serde::json::serde_json::Value as JsonValue;
use rocket::Response;
use rocket::serde::{Serialize, Deserialize};
use rocket::request::Form;

// 定义一个请求参数结构体
#[derive(Deserialize)]
pub struct CheckParams {
    host: String,
    port: u16,
}

// 定义一个响应体结构体
#[derive(Serialize)]
pub struct CheckResponse {
    success: bool,
    message: String,
}

// 定义网络状态检查的函数
fn check_network_status(host: &str, port: u16) -> Result<CheckResponse, String> {
    match TcpStream::connect((host, port)) {
        Ok(_) => Ok(CheckResponse {
            success: true,
            message: format!("Connection to {}:{} is successful", host, port),
        }),
        Err(e) => Err(format!("Failed to connect to {}:{}. Error: {}", host, port, e)),
    }
}

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![check_status])
}

// 定义检查网络状态的路由
#[get("/check_status?<params>")]
fn check_status(params: Form<CheckParams>) -> Result<Json<CheckResponse>, Status> {
    match check_network_status(&params.host, params.port) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(Status::InternalServerError),
    }
}

// 使用ROCKET框架的文档注释
/// 检查网络连接状态的接口
///
/// # GET /check_status
///
/// 传入参数:
/// * `host` - 目标主机地址
/// * `port` - 目标端口号
///
/// ## Response 200
///
/// * `success` - 连接状态
/// * `message` - 连接状态详细信息
///
/// ## Response 500
///
/// * 错误信息
