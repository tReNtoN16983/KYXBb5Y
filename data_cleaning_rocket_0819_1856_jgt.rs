use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use thiserror::Error as ThisError;
use rocket::response::status::BadRequest;

// 自定义错误类型
#[derive(Debug, ThisError)]
pub enum DataCleaningError {
    #[error("Invalid input data: {0}")]
    InvalidInputData(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Other error: {0}")]
    OtherError(String),
}

// 定义输入数据结构
#[derive(Deserialize, Serialize, Debug)]
pub struct InputData {
    pub data: String,
}

// 定义输出数据结构
#[derive(Serialize, Debug)]
pub struct CleanedData {
    pub cleaned_data: String,
}

// 数据清洗和预处理方法
pub fn clean_and_preprocess(data: &str) -> Result<String, DataCleaningError> {
    // 这里添加具体的数据清洗和预处理逻辑
    // 例如：去除前后空白、替换特殊字符、转换数据格式等
    // 以下仅为示例，实际逻辑根据具体需求实现
    let cleaned_data = data.trim().to_string();
    Ok(cleaned_data)
}

// Rocket路由处理函数
#[get("/clean")]
async fn clean_data(input: Json<InputData>) -> Result<Json<CleanedData>, BadRequest<Json<Error>>> {
    let cleaned_data = clean_and_preprocess(&input.data).map_err(|e| {
        BadRequest::new(
            Json::from(serde_json::json!({
                "error": format!("{}", e)
            }))
        )
    })?;
    
    Ok(Json(CleanedData { cleaned_data: cleaned_data }))
}

// 程序入口点
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![clean_data])
}

impl fmt::Display for DataCleaningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DataCleaningError::InvalidInputData(ref err) => write!(f, "{}", err),
            DataCleaningError::IoError(ref err) => write!(f, "{}", err),
            DataCleaningError::OtherError(ref err) => write!(f, "{}", err),
        }
    }
}
