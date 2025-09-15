use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义消息通知请求体
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NotificationRequest {
    message: String,
}

// 定义消息通知响应体
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NotificationResponse {
    status: String,
    message: String,
}

// 消息通知服务状态
struct NotificationService {
    messages: Mutex<HashMap<String, Vec<String>>>, // 存储消息
}

// 实现消息服务状态
impl NotificationService {
    // 初始化消息服务
    pub fn new() -> Self {
        NotificationService {
            messages: Mutex::new(HashMap::new()),
        }
    }

    // 发送消息
    pub fn send_message(&self, user_id: String, message: String) {
        let mut messages = self.messages.lock().unwrap();
        messages.entry(user_id).or_insert_with(Vec::new).push(message);
    }

    // 获取用户消息
    pub fn get_messages(&self, user_id: String) -> Vec<String> {
        let messages = self.messages.lock().unwrap();
        messages.get(&user_id).cloned().unwrap_or_else(Vec::new)
    }
}

// 启动ROCKET应用
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let notification_service = NotificationService::new();
    rocket::build()
        .mount("/", routes![send_notification, get_notifications])
        .manage(notification_service)
        .launch()
        .await?;
    Ok(())
}

// 发送通知的路由
#[get("/send_notification/<user_id>")]
fn send_notification(user_id: String, body: Json<NotificationRequest>,
                    notification_service: &State<NotificationService>) -> Json<NotificationResponse> {
    notification_service.send_message(user_id.clone(), body.message.clone());
    Json(NotificationResponse {
        status: "success".to_string(),
        message: format!("Message sent to user {}", user_id),
    })
}

// 获取通知的路由
#[get("/get_notifications/<user_id>")]
fn get_notifications(user_id: String, notification_service: &State<NotificationService>) -> Json<NotificationResponse> {
    let messages = notification_service.get_messages(user_id.clone());
    Json(NotificationResponse {
        status: "success".to_string(),
        message: format!("Retrieved {} messages for user {}", messages.len(), user_id),
    })
}
