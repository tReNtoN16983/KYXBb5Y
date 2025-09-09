#[macro_use]
extern crate rocket;
# 添加错误处理

use reqwest;
use rocket::Route;
use std::error::Error;
use std::fmt;

// Custom error type for web scraping errors
#[derive(Debug)]
struct ScrapeError {
    message: String,
}
# 扩展功能模块

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ScrapeError {}

impl From<reqwest::Error> for ScrapeError {
    fn from(_: reqwest::Error) -> ScrapeError {
        ScrapeError {
            message: "Failed to make a request to the web page.".to_string(),
        }
    }
}

// Rocket route handler to scrape a web page
#[get("/<url>")]
fn scrape(url: String) -> Result<String, ScrapeError> {
    // Attempt to get the webpage content
    let response = reqwest::blocking::get(&url).map_err(ScrapeError::from);
# 改进用户体验
    
    // Check if the request was successful
    let content = match response {
        Ok(resp) => {
            if resp.status().is_success() {
                resp.text().map_err(ScrapeError::from)?
            } else {
                return Err(ScrapeError {
                    message: format!("Server responded with status: {}", resp.status()),
# 优化算法效率
                })
            }
        },
        Err(_) => return Err(ScrapeError {
            message: "Failed to make a request to the web page.".to_string(),
        }),
    };
    
    // Return the webpage content as a String
    Ok(content)
# 添加错误处理
}

// Main function to set up the Rocket server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![scrape])
}

// Define the routes available
fn routes() -> Vec<Route> {
    routes![scrape]
# 添加错误处理
}
