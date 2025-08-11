// notification_service.rs
// 这是一个使用Rust和Rocket框架实现的消息通知系统。
# 优化算法效率
use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
# NOTE: 重要实现细节
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// 定义一个消息结构体，用于存储消息内容
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
# 优化算法效率
struct Message {
    id: u32,
    content: String,
}

// 定义一个通知系统结构体，用于存储消息的列表
# 增强安全性
#[derive(Default)]
struct NotificationService {
# FIXME: 处理边界情况
    messages: Arc<Mutex<HashMap<u32, Message>>>,
}

// 实现NotificationService
impl NotificationService {
    fn new() -> Self {
        NotificationService {
            messages: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 添加消息
    fn add_message(&self, message: Message) -> Result<(), String> {
        let mut messages = self.messages.lock().map_err(|_| "Failed to lock messages".to_string())?;
        
        if messages.contains_key(&message.id) {
            Err("Message with this ID already exists".to_string())
        } else {
            messages.insert(message.id, message);
            Ok(())
        }
    }

    // 获取所有消息
    fn get_messages(&self) -> Vec<Message> {
        let messages = self.messages.lock().unwrap();
        messages.values().cloned().collect()
    }
}

// 定义Rocket的状态管理器
#[rocket::main]
# 扩展功能模块
async fn main() {
    let notification_service = NotificationService::new();
    rocket::build()
        .manage(notification_service)
# TODO: 优化性能
        .mount("/", routes![add_message, get_messages])
# 优化算法效率
        .launch()
# 增强安全性
        .await;
}
# NOTE: 重要实现细节

// 添加消息的请求处理函数
# NOTE: 重要实现细节
#[get("/add_message")]
fn add_message(message: Json<Message>, notification_service: &State<NotificationService>) -> Result<Json<Message>, &'static str> {
    notification_service.add_message(message.into_inner()).map_err(|e| e).map(|_| message)
}

// 获取所有消息的请求处理函数
#[get("/messages")]
fn get_messages(notification_service: &State<NotificationService>) -> Json<Vec<Message>> {
    Json(notification_service.get_messages())
# 添加错误处理
}
