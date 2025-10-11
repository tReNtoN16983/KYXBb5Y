use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Serialize, Deserialize};

// 定义时间序列数据点结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TimeSeriesDataPoint {
    timestamp: String,
    value: f64,
}

// 定义时间序列数据库接口
trait TimeSeriesDatabase {
    fn insert_data_point(&self, data_point: &TimeSeriesDataPoint) -> Result<(), String>;
    fn query_data_points(&self, start: &str, end: &str) -> Result<Vec<TimeSeriesDataPoint>, String>;
}

// 实现一个简单的内存时间序列数据库
struct InMemoryTimeSeriesDatabase {
    data_points: Vec<TimeSeriesDataPoint>,
}

impl TimeSeriesDatabase for InMemoryTimeSeriesDatabase {
    fn insert_data_point(&self, data_point: &TimeSeriesDataPoint) -> Result<(), String> {
        self.data_points.push(data_point.clone());
        Ok(())
    }

    fn query_data_points(&self, start: &str, end: &str) -> Result<Vec<TimeSeriesDataPoint>, String> {
        let start = start.parse::<i64>().map_err(|_| "Invalid start timestamp")?;
        let end = end.parse::<i64>().map_err(|_| "Invalid end timestamp")?;

        let filtered_data_points = self.data_points.iter()
            .filter(|dp| dp.timestamp.parse::<i64>().map(|ts| ts >= start && ts <= end).unwrap_or(false))
            .cloned()
            .collect();

        Ok(filtered_data_points)
    }
}

// 定义ROCKET路由
#[macro_use] extern crate rocket;

#[get("/insert")]
fn insert(data_point: Json<TimeSeriesDataPoint>) -> Result<Status, String> {
    let db = InMemoryTimeSeriesDatabase { data_points: Vec::new() };
    db.insert_data_point(&data_point.into_inner()).map_err(|e| e).map(|_| Status::Ok)
}

#[get("/query?<start>&<end>")]
fn query(start: String, end: String) -> Result<Json<Vec<TimeSeriesDataPoint>>, String> {
    let db = InMemoryTimeSeriesDatabase { data_points: Vec::new() };
    db.query_data_points(&start, &end).map(Json).map_err(|e| e)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![insert, query])
}