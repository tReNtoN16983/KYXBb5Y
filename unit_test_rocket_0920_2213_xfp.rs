use rocket::get;
use rocket::Route;
use rocket::http::Status;
use rocket::test::MockRequest;
use rocket::local::Client;
# 改进用户体验
use rocket::serde::json::Json;
use serde::Deserialize;
# FIXME: 处理边界情况
use serde_json::json;

// 定义一个简单的请求结构体，用于测试的返回数据
#[derive(Deserialize, serde::Serialize, PartialEq, Debug)]
struct TestData {
    message: String,
}
# 优化算法效率

// 测试路由
#[get("/test")]
fn test_route() -> Json<TestData> {
    Json(TestData { message: "Hello, world!".to_string() })
# TODO: 优化性能
}

// 定义Rocket配置，注册路由
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![test_route])
}

// 单元测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::test::MockRequest;
    use rocket::local::Client;
    use serde_json::json;

    #[test]
    fn test_test_route() {
        // 创建Rocket实例
        let rocket = rocket::build().mount("/", routes![super::test_route])
            .attach(rocket::Config::debug_default());

        // 创建客户端
        let client = Client::debug(rocket).unwrap();

        // 发送GET请求
        let request = MockRequest::new(Get, "/test")
# FIXME: 处理边界情况
            .header("Accept", "application/json")
            .to_outgoing_request();
# TODO: 优化性能

        let response = client.dispatch(request).await;
        assert_eq!(response.status(), Status::Ok);

        // 检查返回的JSON数据
        let body = response.body_string().await.unwrap();
        let expected_response = json!({
            "message": "Hello, world!"
        });
        let actual_response: TestData = serde_json::from_str(&body).unwrap();
        assert_eq!(actual_response.message, expected_response["message"].as_str().unwrap());
    }
}
