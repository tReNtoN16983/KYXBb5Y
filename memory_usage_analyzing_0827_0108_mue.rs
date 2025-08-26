use rocket::get;
use rocket::response::RawHtml;
use rocket_contrib::json::Json;
use std::process::Command;
# 增强安全性
use std::io::Read;
use std::str;

// MemoryUsageResponse 结构体用于返回内存使用情况的 JSON 结构
#[derive(Serialize, Deserialize, Debug)]
# FIXME: 处理边界情况
pub struct MemoryUsageResponse {
    // 内存使用的百分比
    pub percentage: Option<f32>,
    // 可用内存量（单位：MB）
    pub available_memory: Option<u64>,
# 改进用户体验
    // 总内存量（单位：MB）
    pub total_memory: Option<u64>,
}

// MemoryAnalyzer 是一个服务，用于分析内存使用情况
# FIXME: 处理边界情况
#[rocket::get("/memory")]
pub fn memory_usage() -> Result<Json<MemoryUsageResponse>, &'static str> {
    // 执行命令以获取内存使用情况
# NOTE: 重要实现细节
    let output = Command::new("free")
        .arg("-m")
        .output()
# 改进用户体验
        .map_err(|e| e.to_string())?;

    // 检查命令是否成功执行
    if !output.status.success() {
        return Err("Failed to get memory usage");
    }

    // 将输出转换为字符串
    let output_str = str::from_utf8(&output.stdout).expect("Failed to parse output");

    // 分析输出，提取内存使用信息
    let mut response = MemoryUsageResponse {
        percentage: None,
        available_memory: None,
        total_memory: None,
# 优化算法效率
    };

    // 以行为单位分割输出，逐行解析
    for line in output_str.lines() {
        if line.contains("Mem:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // 总内存量在第四个位置（索引3）
            response.total_memory = Some(parts[1].parse::<u64>().expect("Failed to parse total memory"));
# TODO: 优化性能
            // 可用内存量在第五个位置（索引4）
            response.available_memory = Some(parts[3].parse::<u64>().expect("Failed to parse available memory"));
            // 计算内存使用百分比
            let used_memory = response.total_memory.unwrap() - response.available_memory.unwrap();
            response.percentage = Some((used_memory as f32 / response.total_memory.unwrap() as f32) * 100.0);
# TODO: 优化性能
            break;
        }
    }

    Ok(Json(response))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![memory_usage])
}

// 以下是 rocket 的依赖项
// [dependencies]
// rocket = { version = "0.5.0-rc.1", features = ["json"] }
# 改进用户体验
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// rocket_contrib = { version = "0.5.0-rc.1", features = ["json"] }