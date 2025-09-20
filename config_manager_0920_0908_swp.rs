use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::collections::HashMap;
# 扩展功能模块
use std::fs;
use std::io::Result as IoResult;
use std::path::Path;
# TODO: 优化性能
use std::sync::Mutex;
use toml::Value;
# FIXME: 处理边界情况

// 定义配置项结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConfigItem {
    key: String,
    value: String,
}

// 定义配置文件管理器
struct ConfigManager {
    config: Mutex<HashMap<String, String>>,
}

#[rocket::main]
async fn main() {
    // 从指定路径加载配置文件
    let config_path = "config.toml";
    let config_manager = ConfigManager::new(config_path).await;
# FIXME: 处理边界情况
    let config_manager = rocket::fairing::AdHoc::on_attach("ConfigManager", move |rocket| {
        rocket.manage(config_manager)
    });

    rocket.launch().await;
}

impl ConfigManager {
    // 构造函数，加载配置文件
    async fn new(config_path: &str) -> Self {
        let config = ConfigManager::load_config(config_path).expect("Failed to load config");
        ConfigManager {
            config: Mutex::new(config),
        }
# NOTE: 重要实现细节
    }

    // 从文件加载配置
    fn load_config(config_path: &str) -> IoResult<HashMap<String, String>> {
        let content = fs::read_to_string(config_path)?;
        let toml_value: Value = toml::from_str(&content)?;
        let mut config = HashMap::new();
        for (key, value) in toml_value.as_table().unwrap().iter() {
            config.insert(key.clone(), value.as_str().unwrap().to_string());
        }
        Ok(config)
    }

    // 获取配置项
    async fn get_config(&self, key: &str) -> Option<String> {
        let config = self.config.lock().unwrap();
        config.get(key).cloned()
    }
}

#[rocket::get("/config/<key>")]
fn get_config_item(key: String, config_manager: &State<ConfigManager>) -> Option<String> {
    config_manager.get_config(&key).await
}
