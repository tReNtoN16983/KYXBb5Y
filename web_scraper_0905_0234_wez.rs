// web_scraper.rs
// 一个使用RUST和ROCKET框架的网页内容抓取工具。

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use reqwest;
use std::error::Error;
use std::fmt;

#[derive(Debug, serde::Serialize)]
struct ScrapeError {
    message: String,
}

// 定义WebScraper错误类型
#[derive(Debug)]
enum WebScraperError {
    ReqwestError(reqwest::Error),
    UrlParseError(url::ParseError),
    IoError(std::io::Error),
    InvalidResponse,
}

impl fmt::Display for WebScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebScraperError::ReqwestError(e) => write!(f, "Reqwest error: \{e}"),
            WebScraperError::UrlParseError(e) => write!(f, "URL parse error: \{e}"),
            WebScraperError::IoError(e) => write!(f, "IO error: \{e}"),
            WebScraperError::InvalidResponse => write!(f, "Invalid response from server"),
        }
    }
}

impl Error for WebScraperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WebScraperError::ReqwestError(e) => Some(e),
            WebScraperError::UrlParseError(e) => Some(e),
            WebScraperError::IoError(e) => Some(e),
            WebScraperError::InvalidResponse => None,
        }
    }
}

impl From<reqwest::Error> for WebScraperError {
    fn from(error: reqwest::Error) -> Self {
        WebScraperError::ReqwestError(error)
    }
}

impl From<url::ParseError> for WebScraperError {
    fn from(error: url::ParseError) -> Self {
        WebScraperError::UrlParseError(error)
    }
}

impl From<std::io::Error> for WebScraperError {
    fn from(error: std::io::Error) -> Self {
        WebScraperError::IoError(error)
    }
}

// 实现网页内容抓取的功能
async fn scrape_content(url: &str) -> Result<String, WebScraperError> {
    let url = url.parse::<url::Url>()?;

    let response = reqwest::get(url).await?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(WebScraperError::InvalidResponse);
    }

    let content = response.text().await?;
    Ok(content)
}

#[post("/scrape", format = "json", data = "<url>")]
async fn scrape(mut url: String) -> Result<Json<ScrapeError>, status::Custom<ScrapeError>> {
    match scrape_content(&url).await {
        Ok(content) => Ok(Json(ScrapeError { message: content })),
        Err(e) => Err(status::Custom(Status::InternalServerError, ScrapeError { message: e.to_string() })),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![scrape])
}
