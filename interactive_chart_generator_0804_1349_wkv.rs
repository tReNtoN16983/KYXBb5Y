use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;

// 定义一个全局的图表配置存储，使用Mutex确保线程安全
static GLOBAL_CONFIG: Lazy<Arc<Mutex<HashMap<String, Value>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// 图表配置请求结构体
#[derive(Deserialize)]
struct ChartConfig {
    id: String,
    data: Value,
}

// 图表配置响应结构体
#[derive(Serialize)]
struct ChartResponse {
    id: String,
    config: Value,
    status: String,
}

// 设置图表配置的请求处理器
#[post("/charts", format = "json", data = "<chart_config>")]
async fn create_chart(chart_config: Json<ChartConfig>, _db: State<HashMap<String, Value>>) -> Json<ChartResponse> {
    let config = chart_config.into_inner();
    let mut db = _db.lock().unwrap();
    db.insert(config.id.clone(), config.data.clone());

    Json(ChartResponse {
        id: config.id,
        config: config.data,
        status: "success".to_string(),
    })
}

// 获取图表配置的请求处理器
#[get("/charts/<id>")]
async fn get_chart(id: String, _db: State<HashMap<String, Value>>) -> Result<Json<ChartResponse>, Status> {
    let db = _db.lock().unwrap();
    match db.get(&id) {
        Some(config) => Ok(Json(ChartResponse {
            id: id.clone(),
            config: config.clone(),
            status: "success".to_string(),
        })),
        None => Err(Status::NotFound),
    }
}

// 更新图表配置的请求处理器
#[put("/charts/<id>", format = "json", data = "<new_config>")]
async fn update_chart(id: String, new_config: Json<Value>, _db: State<HashMap<String, Value>>) -> Result<Json<ChartResponse>, Status> {
    let config = new_config.into_inner();
    let mut db = _db.lock().unwrap();
    match db.get_mut(&id) {
        Some(config) => {
            *config = config.clone();
            Ok(Json(ChartResponse {
                id: id.clone(),
                config: config.clone(),
                status: "success".to_string(),
            }))
        },
        None => Err(Status::NotFound),
    }
}

// 删除图表配置的请求处理器
#[delete("/charts/<id>")]
async fn delete_chart(id: String, _db: State<HashMap<String, Value>>) -> Result<Json<ChartResponse>, Status> {
    let mut db = _db.lock().unwrap();
    match db.remove(&id) {
        Some(config) => Ok(Json(ChartResponse {
            id: id.clone(),
            config: config,
            status: "success".to_string(),
        })),
        None => Err(Status::NotFound),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(GLOBAL_CONFIG.clone())
        .mount("/api", routes![create_chart, get_chart, update_chart, delete_chart])
}

// 注意：实际部署时需要考虑数据库连接、认证授权、错误日志记录等
