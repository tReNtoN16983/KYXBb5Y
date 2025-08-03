use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use serde_json::json;

// 定义配置文件的结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    settings: HashMap<String, String>,
}

// 配置管理器
struct ConfigManager {
    file_path: String,
    config: Config,
}

#[get("/config")]
// 获取配置文件内容
fn get_config(config_manager: &rocket::State<ConfigManager>) -> String {
    let config_json = serde_json::to_string(&config_manager.config).expect("Failed to serialize config");
    config_json
}

#[get("/config/<key>")]
// 获取配置文件中的特定设置值
fn get_config_value(key: String, config_manager: &rocket::State<ConfigManager>) -> String {
    let value = config_manager.config.settings.get(&key).cloned().unwrap_or_else(|| "".to_string());
    value
}

#[post("/config", format = "json", data = "<config>")]
// 更新配置文件内容
fn update_config(config: Config, config_manager: rocket::State<ConfigManager>) -> String {
    config_manager.config = config;
    match write_config(&config_manager.file_path, &config_manager.config) {
        Ok(_) => "Config updated successfully".to_string(),
        Err(e) => format!("Failed to update config: {}", e),
    }
}

// 读取配置文件
fn read_config(file_path: &str) -> io::Result<Config> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// 写入配置文件
fn write_config(file_path: &str, config: &Config) -> io::Result<()> {
    let config_json = serde_json::to_string(config)?;
    let mut file = File::create(file_path)?;
    file.write_all(config_json.as_bytes())
}

#[launch]
// 启动Rocket应用
fn rocket() -> _ {
    let file_path = "config.json".to_string();
    let config = read_config(&file_path).expect("Failed to read config file");
    rocket::build()
        .manage(ConfigManager { file_path, config })
        .mount("/", routes![get_config, get_config_value, update_config])
}
