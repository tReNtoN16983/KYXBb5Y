use rocket::get;
use rocket::http::{Status, ContentType};
use rocket::response::{self, Responder, Response, status::Custom};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

// 定义一个简单的用户模型
#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

// 定义一个认证请求体
#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

// 定义认证响应体
#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
}

// 模拟用户存储，实际应用中应使用数据库
lazy_static::lazy_static! {
    static ref USERS: std::sync::Mutex<std::collections::HashMap<String, String>> = {
        let mut m = std::collections::HashMap::new();
        m.insert(