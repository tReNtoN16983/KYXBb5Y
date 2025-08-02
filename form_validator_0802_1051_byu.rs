use rocket::form::Form;
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::{self, Request, FromRequest};
use rocket::response::status;
use std::fmt;
use std::ops::Deref;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

// 定义一个表单结构体，用于验证和序列化数据
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MyForm<'r> {
    #[serde(borrow)]
    name: &'r str,
    age: u8,
}

// 自定义错误类型
#[derive(Error, Debug)]
pub enum FormError {
    #[error("invalid form data")]
    InvalidFormData,
    #[error("missing form field: {0}")]
    MissingField(String),
    #[error("validation failed: {0}")]
    Validation(String),
}

// 实现FromRequest trait，用于将表单数据解析为MyForm
#[rocket::async_trait]
impl<'r> FromRequest<'r> for MyForm<'r> {
    type Error = FormError;

    async fn from_request(request: &'r Request<'_), mut input: request::Payload<'_>) -> request::Outcome<Self, Self::Error> {
        let form = match Form::parse(&request, &mut input).await {
            Ok(form) => form,
            Err(_) => return Failure((Status::BadRequest, FormError::InvalidFormData)),
        };

        let name = form.get("name")
            .ok_or_else(|| FormError::MissingField("name".to_string()))?
            .as_str()
            .ok_or_else(|| FormError::InvalidFormData)?;

        let age = form.get("age")
            .ok_or_else(|| FormError::MissingField("age".to_string()))?
            .as_u64()
            .and_then(|age| age.checked_sub(0).filter(|&age| age < 200).ok_or(FormError::Validation("age must be between 0 and 200".to_string())))
            .ok_or_else(|| FormError::InvalidFormData)?
            as u8;

        Ok(Success(MyForm { name, age }))
    }
}

// 示例用法：
#[post("/form", data = "<form>")]
async fn form_handler(form: MyForm) -> status::Accepted<&'static str> {
    println!("Received form with name: {}, age: {}", form.name, form.age);
    status::Accepted("Form received")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![form_handler])
}
