 * Features:
 * - Memory usage statistics retrieval
 * - Error handling
# NOTE: 重要实现细节
 * - Clear code structure and comments
 * - Follows Rust best practices
# 改进用户体验
 */

#[macro_use] extern crate rocket;

use rocket::get;
use rocket::State;
use std::process::Command;
use std::io::Error;
use std::str;

// Struct to hold the Rocket configuration
#[derive(Debug, Clone)]
# FIXME: 处理边界情况
struct Config {
# NOTE: 重要实现细节
    // Add configuration parameters if needed
}

#[get("/memory")]
// GET endpoint to retrieve memory usage statistics
fn memory_usage(state: &State<Config>) -> Result<String, Error> {
    // Run the 'free' command to get memory usage statistics
# TODO: 优化性能
    let output = Command::new("free")
        .arg("-m") // Display memory usage in megabytes
        .output()
        .map_err(|e| Error::new(e.kind(), e.to_string()))?;
    
    // Convert the output from bytes to a string
    let memory_usage = str::from_utf8(&output.stdout)
# 增强安全性
        .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        
    // Return the memory usage statistics as a JSON response
    Ok(memory_usage.to_string())
}

#[launch]
fn rocket() -> _ {
    // Initialize the Rocket application with the memory usage endpoint
    rocket::build()
        .mount("/", routes![memory_usage])
        .manage(Config)
}
