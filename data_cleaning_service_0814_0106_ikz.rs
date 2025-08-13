use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 定义一个数据清洗和预处理的请求结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct DataCleaningRequest {
    pub data: Vec<String>,
}

// 定义一个数据清洗和预处理的响应结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct DataCleaningResponse {
    pub cleaned_data: Vec<String>,
}

// 实现数据清洗和预处理的业务逻辑
pub struct DataCleaningService;

impl DataCleaningService {
    // 提供一个清洗数据的方法
    pub fn clean_data(&self, request: &DataCleaningRequest) -> DataCleaningResponse {
        let mut cleaned_data = Vec::new();

        // 遍历请求中的数据
        for item in &request.data {
            // 这里添加具体的数据清洗逻辑，例如去除空格、转换为小写等
            let cleaned_item = item.trim().to_lowercase();
            cleaned_data.push(cleaned_item);
        }

        DataCleaningResponse {
            cleaned_data,
        }
    }
}

// 定义一个控制器，用于处理HTTP请求
#[macro_use] extern crate rocket;

#[get("/clean_data")]
fn clean_data_controller() -> Json<DataCleaningResponse> {
    let request = DataCleaningRequest {
        data: vec![
            "  Hello World  ",
            "Rust Programming Language".to_string(),
            "ROCKET Framework".to_string(),
        ],
    };

    let service = DataCleaningService;
    let response = service.clean_data(&request);

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![clean_data_controller])
}
