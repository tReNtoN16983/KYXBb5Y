use rocket::get;
use rocket::http::Status;
use rocket::response::content;
use rocket::serde::json::Json;
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::Read;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use regex::Regex;
use lazy_static::lazy_static;
use rocket::State;
use rocket::Outcome;
use rocket::Request;
use rocket::Response;
use rocket::http::RawStr;

#[macro_use]
extern crate lazy_static;

// 全局配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    pub user_agent: String,
    pub timeout: u64,
}

lazy_static! {
    static ref GLOBAL_CONFIG: Mutex<GlobalConfig> = Mutex::new(GlobalConfig {
        user_agent: "Rust-Web-Scraper".to_string(),
        timeout: 30,
    });
}

// 网页内容抓取请求
#[derive(Debug, Clone, Deserialize)]
pub struct ScrapeRequest {
    pub url: String,
}

// 网页内容抓取响应
#[derive(Debug, Serialize)]
pub struct ScrapeResponse {
    pub status: String,
    pub content: Option<String>,
    pub error: Option<String>,
}

// 抓取网页内容的错误类型
#[derive(Debug)]
pub enum ScrapeError {
    ConnectError(reqwest::Error),
    ReadError(std::io::Error),
    TimeoutError,
    RegexError(regex::Error),
}

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrapeError::ConnectError(e) => write!(f, "Connect error: {}