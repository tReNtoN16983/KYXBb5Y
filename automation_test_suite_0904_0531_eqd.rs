// automation_test_suite.rs
//
// 这是一个自动化测试套件，使用RUST和ROCKET框架实现。

#[macro_use]
extern crate rocket;

// 引入rocket框架的测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::serde::json::Json;
    use serde_json::json;

    // 测试我们的根端点
    #[test]
    fn test_get_root() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        
        // 断言状态码是200
        assert_eq!(response.status(), Status::Ok);
        // 断言返回的JSON内容
        assert_eq!(response.body_string(), Some("Hello, Rust!".to_string()));
    }

    // 测试我们的健康状况检查端点
    #[test]
    fn test_health_check() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/health").dispatch();
        
        // 断言状态码是200
        assert_eq!(response.status(), Status::Ok);
        // 断言返回的JSON内容
        assert_eq!(response.body_string(), Some("I am healthy".to_string()));
    }

    // 测试我们的API端点，它需要一个JSON对象作为输入
    #[test]
    fn test_api_endpoint() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let json_data = Json::serde_json::to_string(&json!({"key": "value"})).expect("valid json