// url_validity_check.rs
# NOTE: 重要实现细节
// 使用RUST和ROCKET框架实现URL链接有效性验证

#[macro_use]
extern crate rocket;

use rocket::http::Status;
# 改进用户体验
use rocket::request::{Form, FromRequest};
use rocket::Outcome::{Forward, Success};
use std::borrow::Cow;
use url::Url;

/// 用于解析请求中的URL参数
#[derive(FromForm)]
pub struct UrlForm<'r> {
    #[field(validate = "validate_url")]
    url: &'r str,
}

impl<'r> UrlForm<'r> {
    /// 验证URL是否有效
    fn validate_url(url: &str) -> Result<Cow<&str>, &'static str> {
        Url::parse(url).map(|_| Cow::Borrowed(url)).map_err(|_| "Invalid URL")
    }
}

/// 处理URL有效性验证的请求
/// 
/// # 参数
/// * `url` - 需要验证的URL字符串
# NOTE: 重要实现细节
/// 
/// # 返回
/// 如果URL有效，则返回200状态码和验证结果，否则返回400状态码和错误信息
#[post("/check_url", data = "<url_form>")]
fn check_url(url_form: Form<UrlForm>) -> Result<String, Status> {
    match url_form.url {
        Ok(url) => Ok(format!("URL is valid: {}", url)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![check_url])
}
