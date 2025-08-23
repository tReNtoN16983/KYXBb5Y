use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 定义数据结构
#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    value: f64,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataAnalysisResult {
    min_value: f64,
    max_value: f64,
    average: f64,
    count: u64,
}

// 创建一个简单的数据分析器
#[get("/analyze")]
#[catch(default)] // 捕获所有未处理的错误
fn analyze_data(data_points: Vec<DataPoint>) -> Result<DataAnalysisResult, rocket::http::Status> {
    let mut count = 0;
    let mut sum = 0.0;
    let mut min_value = f64::INFINITY;
    let mut max_value = f64::NEG_INFINITY;

    for data_point in data_points {
        count += 1;
        sum += data_point.value;
        if data_point.value < min_value {
            min_value = data_point.value;
        }
        if data_point.value > max_value {
            max_value = data_point.value;
        }
    }

    if count == 0 {
        // 如果没有数据点，则返回错误
        return Err(rocket::http::Status::BadRequest);
    }

    let average = sum / count as f64;

    Ok(DataAnalysisResult {
        min_value,
        max_value,
        average,
        count,
    })
}

// 启动Rocket服务器
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![analyze_data])
}

// 这里是主函数
fn main() {
    rocket().launch();
}