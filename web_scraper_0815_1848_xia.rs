use rocket::get;
use rocket::Route;
use rocket::serde::json::Json;
use reqwest;
use std::error::Error;
use std::io::Read;
use scraper::Html;
use scraper::Selector;
use serde::Serialize;
use rocket::http::Status;

#[macro_use]
extern crate rocket;

// 定义返回结构体
#[derive(Debug, Serialize)]
struct ScrapedContent {
    url: String,
    content: String,
}

// 定义错误枚举
#[derive(Debug)]
enum ScraperError {
    HttpError(reqwest::Error),
    HtmlParseError,
}

// 将错误转换为适合火箭的错误处理形式
impl<'r> rocket::response::Responder<'r, 'static> for ScraperError {
    fn respond_to(self, _: &'r rocket::Request) -> rocket::response::Result<'static> {
        match self {
            ScraperError::HttpError(err) => Err(Status::BadGateway),
            ScraperError::HtmlParseError => Err(Status::InternalServerError),
        }
    }
}

// 主程序结构
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![scrape])
}

// 定义路由
#[get("/scrape<url>")]
fn scrape(url: String) -> Result<Json<ScrapedContent>, ScraperError> {
    let response = reqwest::blocking::get(&url).map_err(ScraperError::HttpError)?;
    let body = response.text().map_err(|_| ScraperError::HtmlParseError)?;
    let document = Html::parse_document(&body);

    // 选择器用于选取网页内容，这里只是一个示例，需要根据实际网页结构进行调整
    let selector = Selector::parse("body").unwrap();
    let content = document.select(&selector)
        .filter_map(|element| element.text())
        .collect::<Vec<_>>()
        .join("
");

    Ok(Json(ScrapedContent {
        url,
        content,
    }))
}

// 使用ROCKET自定义错误处理
#[error]
fn error_handler<'r>(err: &'r ScraperError, _: &'r rocket::Request) -> rocket::response::Result<'r> {
    match err {
        ScraperError::HttpError(_) => rocket::Response::build().status(Status::BadGateway).body("Failed to fetch URL."),
        ScraperError::HtmlParseError => rocket::Response::build().status(Status::InternalServerError).body("Failed to parse HTML."),
    }
}
