use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use csv::ReaderBuilder;
use std::error::Error;
use rocket::response::status;
use rocket::response::stream::Stream;

// 定义一个结构体来表示CSV文件中的数据行
#[derive(Debug, Deserialize, Serialize)]
struct CsvRow {
    // 根据实际CSV文件的列来定义字段
    column1: String,
    column2: String,
    // 可以添加更多的字段
}

// 定义状态，用于存储输入目录和输出目录的路径
struct BatchProcessor {
    input_dir: String,
    output_dir: String,
}

// 实现BatchProcessor的默认构造函数
impl Default for BatchProcessor {
    fn default() -> Self {
        BatchProcessor {
            input_dir: String::from("./input"),
            output_dir: String::from("./output"),
        }
    }
}

// 创建rocket的API端点，用于处理CSV文件
#[get("/process_csv")]
fn process_csv(batch_processor: &State<BatchProcessor>) -> Result<Json<Vec<CsvRow>>, status::InternalServerError<&'static str>> {
    // 列出输入目录中的CSV文件
    let input_path = Path::new(&batch_processor.input_dir);
    let files = match fs::read_dir(input_path) {
        Ok(files) => files,
        Err(e) => return Err(e.into()),
    };

    // 遍历文件并处理它们
    let mut results = Vec::new();
    for file in files {
        let file = match file {
            Ok(file) => file,
            Err(e) => return Err(e.into()),
        };
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if !file_name.ends_with(".csv") {
            continue;
        }

        // 读取CSV文件并解析内容
        let reader = ReaderBuilder::new().has_headers(true).from_path(path).unwrap();
        for result in reader.deserialize() {
            match result {
                Ok(row) => results.push(row),
                Err(e) => return Err(e.into()),
            }
        }
    }

    // 返回处理结果
    Ok(Json(results))
}

// 定义Rocket的启动函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![process_csv])
        .manage(BatchProcessor::default())
}

// 错误处理：将错误转换为InternalServerError
impl From<csv::Error> for status::InternalServerError<&'static str> {
    fn from(err: csv::Error) -> Self {
        status::InternalServerError(Some(err.description()))
    }
}

// 错误处理：将std::io::Error转换为InternalServerError
impl From<std::io::Error> for status::InternalServerError<&'static str> {
    fn from(err: std::io::Error) -> Self {
        status::InternalServerError(Some(err.description()))
    }
}

// 使用Rocket的main函数
#[cfg(feature = "dev")]
fn main() {
    rocket().launch();
}
