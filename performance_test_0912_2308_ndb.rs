use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Error as SerdeError;
use std::time::Instant;
use std::thread;
use std::time::Duration;

// 定义一个简单的数据结构，用于性能测试结果的返回
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct PerformanceResult {
    operation: String,
    duration: f64,
    status: String,
}

#[get("/perform_test")]
fn perform_test() -> Result<Json<PerformanceResult>, Status> {
    // 开始计时
    let start = Instant::now();
    
    // 性能测试的逻辑
    // 这里我们使用thread::sleep来模拟一个耗时操作
    thread::sleep(Duration::from_secs(1));
    
    // 计算操作耗时
    let duration = start.elapsed().as_secs_f64();
    
    // 创建性能测试结果
    let result = PerformanceResult {
        operation: "sleep".to_string(),
        duration,
        status: "success