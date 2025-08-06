use rocket::Route;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;
use std::sync::Mutex;
use lazy_static::lazy_static;
use regex::Regex;

// 定义用户身份模型
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

// 定义错误响应模型
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// 模拟的用户数据库
lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(vec![
        User { id: 1, username: "admin".to_string(), password: "password123".to_string() },
    ]);
}

// 身份认证服务
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![login])
}

// 登录路由
#[post("/login", format = "json", data = "<user>")]
async fn login(user: Json<User>) -> Result<status::Accepted<&'static str>, status::BadRequest<Json<ErrorResponse>>> {
    // 正则表达式用于验证用户名和密码格式
    let username_regex = Regex::new("^[a-zA-Z0-9_-]{3,16}$").unwrap();
    let password_regex = Regex::new("^[a-zA-Z0-9@#$%^&+=]{6,18}$").unwrap();

    // 验证用户名和密码格式
    if !username_regex.is_match(&user.username) || !password_regex.is_match(&user.password) {
        return Err(status::BadRequest(Some(Json(ErrorResponse { error: "Invalid username or password format".to_string() }))))
    }

    // 检查用户名和密码是否匹配
    let users = USERS.lock().unwrap();
    if let Some(existing_user) = users.iter().find(|u| u.username == user.username && u.password == user.password) {
        Ok(status::Accepted(Some("User authenticated successfully"), ""))
    } else {
        Err(status::BadRequest(Some(Json(ErrorResponse { error: "Invalid username or password".to_string() }))))
    }
}

// 路由定义
pub fn routes() -> Vec<Route> {
    routes![login]
}
