use rocket::get;
use rocket::Route;
use std::process::Command;
use std::str;

/// 获取当前进程的内存使用情况
#[get("/memory")]
/// # 响应
/// 返回当前进程的内存使用情况的JSON对象
fn memory_usage() -> Result<String, std::io::Error> {
    // 执行命令以获取内存使用情况
    let output = Command::new("ps")
        .args("-o rss,vsize,pcpu,pid,user --no-headers -p")
        .output()?;
    
    // 将输出转换为字符串
    let output_str = str::from_utf8(&output.stdout).unwrap();
    
    // 分析输出并提取内存使用情况
    let memory_usage = output_str.lines()
        .next() // 取第一行（当前进程）
        .unwrap_or("")
        .split_whitespace()
        .collect::<Vec<&str>>();
    
    // 构建响应JSON对象
    let memory_info = json!({
        "rss": memory_usage.get(0).unwrap_or(&""),
        "vsize": memory_usage.get(1).unwrap_or(&""),
        "pcpu": memory_usage.get(2).unwrap_or(&""),
        "pid": memory_usage.get(3).unwrap_or(&""),
        "user": memory_usage.get(4).unwrap_or(&"")
    });
    
    // 将JSON对象序列化为字符串
    Ok(memory_info.to_string())
}

/// 定义Rocket路由
#[launch]
fn rocket() -> rocket::Rocket {
    rocket::build().mount("/api", routes![memory_usage])
}

/// 用于序列化JSON对象
#[cfg(not(feature = "json"))]
mod json;

/// 用于序列化JSON对象
#[cfg(feature = "json"))]
extern crate serde_json;
mod json {
    pub fn to_string(value: serde_json::Value) -> String {
        serde_json::to_string(&value).unwrap()
    }
}

/// 用于序列化JSON对象
use json::to_string;
