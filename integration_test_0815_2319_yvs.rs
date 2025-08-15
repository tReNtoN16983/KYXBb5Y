// integration_test.rs
// 这是一个使用ROCKET框架的集成测试文件。
# 增强安全性
use rocket::local::Client;
use rocket::http::Status;
use super::rocket; // 假设rocket函数在lib.rs中定义

#[cfg(test)]
mod tests {
    // 导入所需的模块
    use super::*;

    #[test]
    fn test_home_route() {
        // 创建一个Rocket实例
        let client = Client::new(rocket()).expect("valid rocket instance");

        // 测试根路由
        let mut response = client.get("/").dispatch();

        // 验证状态码和响应体
        assert_eq!(response.status(), Status::Ok);
# 优化算法效率
        assert_eq!(response.body_string(), Some("Hello, world!".to_string()));
    }
# 优化算法效率

    #[test]
    fn test_nonexistent_route() {
        // 测试不存在的路由
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/nonexistent").dispatch();

        // 验证状态码
        assert_eq!(response.status(), Status::NotFound);
    }
}
