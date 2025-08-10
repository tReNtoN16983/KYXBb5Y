use serde::{Deserialize, Serialize};
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

// 定义了一个示例用户的数据模型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "id")]
    pub user_id: u32,
    #[serde(rename = "name")]
    pub user_name: String,
    #[serde(rename = "email")]
    pub email: String,
}

// 实现Rocket表单转换User模型
#[derive(FromForm)]
pub struct UserForm {
    #[form(field = "id")]
    user_id: u32,
    #[form(field = "name")]
    user_name: String,
    #[form(field = "email")]
    email: String,
}

// 错误处理枚举
#[derive(Debug)]
pub enum UserError {
    InvalidInput(String),
    UserNotFound,
}

impl From<UserError> for status::Custom<'static> {
    fn from(err: UserError) -> status::Custom<'static> {
        match err {
            UserError::InvalidInput(msg) => {
                status::Custom(Status::BadRequest, Json(msg))
            },
            UserError::UserNotFound => {
                status::Custom(Status::NotFound, Json("User not found"))
            },
        }
    }
}

// 一个简单的用户服务，用于处理用户相关的逻辑
pub struct UserService;

impl UserService {
    // 创建一个新的用户
    pub fn create_user(user_form: &UserForm) -> Result<Json<User>, UserError> {
        // 这里是用户创建的逻辑，例如数据库操作，现在返回一个简单的Json响应
        Ok(Json(User {
            user_id: user_form.user_id,
            user_name: user_form.user_name.clone(),
            email: user_form.email.clone(),
        }))
    }

    // 获取一个用户
    pub fn get_user(user_id: u32) -> Result<Json<User>, UserError> {
        // 这里是获取用户的逻辑，例如数据库查询
        // 为了示例，我们直接返回Error::UserNotFound
        Err(UserError::UserNotFound)
    }
}
