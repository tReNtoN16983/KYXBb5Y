use rocket::form::Form;
use rocket::http::RawStr;
use rocket::serde::json::Json;
use rocket::State;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, FromRequest};
use rocket::response::status::BadRequest;
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;
use validator::{ValidationErrors, validate_email};

// 定义一个表单数据结构
#[derive(FromForm, Deserialize, Debug)]
pub struct FormData {
    username: String,
    email: String,
    age: u8,
}

// 定义表单数据验证器
pub fn validate_form_data(form_data: &FormData) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    // 验证用户名非空
    if form_data.username.trim().is_empty() {
        errors.add("username", "is required");
    }

    // 验证邮箱格式
    if let Err(e) = validate_email(form_data.email.as_str()) {
        errors.add("email", e.message);
    }

    // 验证年龄为正数
    if form_data.age == 0 {
        errors.add("age", "must be greater than 0");
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(())
    }
}

// 实现 FromRequest trait，用于从请求中解析表单数据
#[rocket::async_trait]
impl<'r> FromRequest<'r> for FormData {
    type Error = BadRequest<Json<ValidationErrors>>;

    async fn from_request(request: &'r Request<'_>, _: &rocket::Data) -> request::Outcome<Self, Self::Error> {
        let form_data = request.guard::<rocket::form::Form<FormData>>().await.into_inner().into_inner();

        match validate_form_data(&form_data) {
            Ok(_) => request::Outcome::Success(form_data),
            Err(errors) => {
                request::Outcome::Failure((
                    BadRequest::with_body(Json(errors)),
                ))
            },
        }
    }
}

// 定义一个简单的表单验证示例路由
#[post("/form", data = "<form_data>")]
pub fn form_data_route(form_data: Json<FormData>, errors: &State<ValidationErrors>) -> Json<FormData> {
    if errors.is_empty() {
        Json(form_data.0)
    } else {
        Json(FormData {
            username: String::from("error"),
            email: String::from("error"),
            age: 0,
        })
    }
}
