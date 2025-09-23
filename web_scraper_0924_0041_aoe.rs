use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use std::error::Error;
use url::Url;
use select::document::Document;
use select::predicate::Name;
use std::io::Read;

#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate url;
extern crate select;

// 定义响应结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct WebScraperResponse {
    pub url: String,
    pub content: String,
}

#[get("/scraper?url=<url>")]
fn scraper(url: String) -> Result<Json<WebScraperResponse>, Status> {
    let client = Client::builder()
        .user_agent("Rust-Scraper")
        .build().unwrap();

    match client.get(&url).send() {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text()?;
                let document = Document::from(&body);

                // 假设我们想要抓取的是body标签中的内容
                let content = document.find(Name("body")).next().unwrap_or_default().text().collect::<Vec<_>>().join("
");

                Ok(Json(WebScraperResponse {
                    url: url.clone(),
                    content,
                }))
            } else {
                Err(Status::InternalServerError)
            }
        },
        Err(e) => {
            eprintln!("Error while scraping: {}", e);
            Err(Status::InternalServerError)
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![scraper])
}

// 主函数，用于启动Rocket服务器
fn main() {
    rocket().launch();
}