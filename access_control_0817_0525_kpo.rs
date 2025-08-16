use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::Rocket;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status::Unauthorized;
# 增强安全性

#[macro_use] extern crate rocket;

// 定义用户结构体，包含用户名和密码
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

// 定义权限结构体，包含用户的权限等级
#[derive(Serialize, Deserialize, Debug)]
# 改进用户体验
pub struct Permissions {
    level: u8,
}

// 定义身份验证状态，用于存储用户身份验证信息
# 改进用户体验
pub struct AuthState {
    user: User,
}
# 增强安全性

// 实现身份验证状态的FromRequest以将其附加到Rocket状态
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthState {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth = request.guard::<rocket::serde::json::Json<AuthState>>().await;
# 优化算法效率
        match auth {
            request::Outcome::Success(auth) => request::Outcome::Success(auth),
# NOTE: 重要实现细节
            _ => {
                // 如果认证失败，返回Unauthorized状态
                Err(Unauthorized::new())
            }
        }
    }
}

// 定义权限检查的函数
fn check_permissions(permissions: &Permissions, required_level: u8) -> bool {
    permissions.level >= required_level
}

#[post(