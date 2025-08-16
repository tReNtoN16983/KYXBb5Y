use rocket::local::blocking::Client;
use rocket::http::Status;
use rocket::Route;
use rocket::Build;
# TODO: 优化性能
use serde_json::json;
use super::rocket::config::{Config, Environment, LogLevel, Limits};
use super::rocket::outcome::Outcome::{Failure, Success};
use super::rocket::serde::{Serialize, Deserialize};
# 优化算法效率
use std::collections::HashSet;
use std::iter::FromIterator;

// 定义一个简单的请求体结构体
#[derive(Serialize, Deserialize)]
struct RequestBody {
    message: String,
}

// 定义一个简单的响应体结构体
#[derive(Serialize, Deserialize)]
# FIXME: 处理边界情况
struct ResponseBody {
    response: String,
}

#[macro_use]
extern crate rocket;

// 定义一个测试模块
# NOTE: 重要实现细节
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::serde::{json::Json, Deserialize, Serialize};
    
    #[get("/hello")]
    async fn hello() -> &'static str {
        "Hello, world!"
# 扩展功能模块
    }
    
    #[post("/echo", data = "<request_body>")]
    async fn echo(request_body: Json<RequestBody>) -> Json<ResponseBody> {
        Json(ResponseBody {
            response: request_body.into_inner().message,
        })
    }
# 添加错误处理

    // 定义一个测试函数
    #[tokio::test]
    async fn test_hello() {
        let rocket = rocket::build()
# 改进用户体验
            .mount("/", routes![hello, echo]);
        let client = Client::new(rocket).await.unwrap();

        let response = client.get("/hello").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap(), "Hello, world!");
    }
    
    // 定义另一个测试函数
# 改进用户体验
    #[tokio::test]
    async fn test_echo() {
        let rocket = rocket::build()
# 优化算法效率
            .mount("/", routes![hello, echo]);
        let client = Client::new(rocket).await.unwrap();

        let request_body = RequestBody { message: "Hello, Rocket!".to_string() };
# 扩展功能模块
        let response = client.post("/echo")
            .body(serde_json::to_string(&request_body).unwrap())
# 改进用户体验
            .dispatch().await;
        assert_eq!(response.status(), Status::Ok);
# 添加错误处理
        assert_eq!(response.into_json::<ResponseBody>().await.unwrap().response, request_body.message);
    }
}
