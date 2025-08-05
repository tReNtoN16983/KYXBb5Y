use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::Request;
use rocket::Outcome;
# 添加错误处理
use rocket::serde::{Serialize, Deserialize};
use rocket::outcome::IntoOutcome;
use rocket::form::FromFormField;
use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

// 数据模型
#[derive(Serialize, Deserialize, Debug)]
struct FormData {
    name: String,
    age: u8,
}

// 表单数据验证器
impl<'r> FromFormField<'r> for FormData {
    fn from_value(field: &'r str) -> Option<Self> {
        // 尝试解析表单数据
        let mut parts = field.splitn(2, '&');
        let name = parts.next()?.split('=').next()?.to_string();
        let age = parts.next()?.split('=').next()?.to_string();

        // 验证和解析年龄
        let age = match age.parse::<u8>() {
# TODO: 优化性能
            Ok(age) => age,
            Err(_) => return None,
        };

        Some(FormData {
            name,
# 改进用户体验
            age,
        })
    }
}

// 错误类型
#[derive(Debug)]
enum FormDataError {
    MissingField,
# NOTE: 重要实现细节
    InvalidAge(ParseIntError),
}

// 错误处理
impl fmt::Display for FormDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormDataError::MissingField => write!(f, "Missing field"),
            FormDataError::InvalidAge(e) => write!(f, "Invalid age: {}", e),
        }
    }
}

impl std::error::Error for FormDataError {}

// 路由处理器
#[post("/form", data = "<form_data>")]
fn form_handler(form_data: Result<Form<FormData>, FormDataError>) -> Json<status::Custom<&'static str>> {
    match form_data {
# 优化算法效率
        Ok(formData) => {
            // 处理有效的表单数据
# 优化算法效率
            Json(status::Custom(Status::Ok, "Form data is valid".to_string()))
        },
        Err(_) => {
            // 处理表单验证错误
            Json(status::Custom(Status::BadRequest, "Invalid form data".to_string()))
        },
    }
}

// Rocket 启动配置
# TODO: 优化性能
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![form_handler])
# NOTE: 重要实现细节
}

/// 该程序是一个使用ROCKET框架的表单数据验证器。
/// 它定义了一个`FormData`结构体来表示表单数据，并实现了`FromFormField` trait
/// 来从表单数据中解析`FormData`。
/// 它还定义了一个`FormDataError`枚举来处理表单验证错误。
/// 路由处理器`form_handler`接受表单数据并返回相应的HTTP状态码。
