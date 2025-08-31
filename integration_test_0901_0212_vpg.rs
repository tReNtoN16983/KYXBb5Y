use rocket::http::Status;
use rocket::local::blocking::Client;
# NOTE: 重要实现细节
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::tokio::io::AsyncWriteExt;

// 定义测试用的请求数据结构
#[derive(serde::Serialize, serde::Deserialize)]
struct TestRequest {
# TODO: 优化性能
    message: String,
}

// 定义测试用的响应数据结构
#[derive(serde::Serialize, serde::Deserialize)]
struct TestResponse {
    echo: String,
}

// 定义Rocket应用
# NOTE: 重要实现细节
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket_contrib;
# FIXME: 处理边界情况

#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use serde_json::json;

    #[test]
    fn test_integration() {
        // 设置Rocket应用
        let rocket = rocket::build()
# NOTE: 重要实现细节
            .mount("/", routes![super::echo_message])
            .attach(
                rocket::Config::new(rocket::config::Environment::Development)
# 添加错误处理
                    .port(8000)
            );

        // 创建测试客户端
        let client = Client::new(rocket).unwrap();

        // 构建请求数据
        let request_data = json!({
# 增强安全性
            "message": "Hello, World!"
        });

        // 发送POST请求
        let response = client.post("/echo")
            .body(request_data.to_string())
            .dispatch();

        // 检查状态码
        assert_eq!(response.status(), Status::Ok);

        // 检查响应内容
        let response_data = response.json::<TestResponse>().unwrap();
# 扩展功能模块
        assert_eq!(response_data.echo, "Hello, World!");
    }
}

// 定义路由处理函数
#[post("/echo", data = "<request_data>")]
fn echo_message(request_data: Json<TestRequest>) -> Json<TestResponse> {
    // 处理请求数据
    let message = request_data.into_inner().message.clone();

    // 返回响应数据
    Json(TestResponse { echo: message })
}
