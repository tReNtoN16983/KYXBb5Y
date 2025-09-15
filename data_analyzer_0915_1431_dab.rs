use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
# 改进用户体验

// 定义一个数据点结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
# 添加错误处理
struct DataPoint {
    value: f64,
    timestamp: String,
}

// 定义统计数据结构体
#[derive(Default, Serialize, Deserialize, Clone)]
struct Statistics {
    count: u32,
    max: f64,
    min: f64,
    sum: f64,
    mean: f64,
}

// 定义数据分析器结构体，包含统计数据
struct DataAnalyzer {
    stats: Mutex<HashMap<String, Statistics>>,
# 扩展功能模块
}

impl DataAnalyzer {
    // 创建一个新的数据分析器
    fn new() -> Self {
        DataAnalyzer {
            stats: Mutex::new(HashMap::new()),
        },
    }

    // 添加数据点
# 改进用户体验
    fn add_data_point(&self, category: &str, data_point: DataPoint) {
        let mut stats = self.stats.lock().unwrap();
        if !stats.contains_key(category) {
            stats.insert(category.to_string(), Statistics::default());
        }
        let stat = stats.get_mut(category).unwrap();
        stat.count += 1;
        stat.max = stat.max.max(data_point.value);
        stat.min = stat.min.min(data_point.value);
        stat.sum += data_point.value;
        stat.mean = stat.sum / stat.count as f64;
    }

    // 获取统计数据
    fn get_statistics(&self, category: &str) -> Option<Statistics> {
# 改进用户体验
        let stats = self.stats.lock().unwrap();
        stats.get(category).cloned()
# 添加错误处理
    }
}

#[get("/add_data_point/<category>")]
fn add_data_point_route(data_analyzer: &State<DataAnalyzer>, category: String, data_point: Json<DataPoint>) -> &'static str {
    data_analyzer.add_data_point(&category, data_point.into_inner());
    "Data point added successfully"
}

#[get("/get_statistics/<category>")]
fn get_statistics_route(data_analyzer: &State<DataAnalyzer>, category: String) -> Option<Json<Statistics>> {
    data_analyzer.get_statistics(&category).map(Json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DataAnalyzer::new())
        .mount("/", routes![add_data_point_route, get_statistics_route])
}
