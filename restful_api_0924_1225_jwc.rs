use rocket::get;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
use rocket::State;
use std::sync::Mutex;
use rocket::fairing::AdHoc;
use rocket::Config;
use std::collections::HashMap;

// 定义数据模型
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 实现User请求的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct GetUsersRequest {
    limit: Option<u32>,
    offset: Option<u32>,
}

// 模拟数据库
lazy_static! {
    static ref DATABASE: Mutex<HashMap<u32, User>> = Mutex::new(
        HashMap::from([
            (1, User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() }),
            (2, User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() }),
            // 可以继续添加更多的测试用户
        ])
    );
}

// 用户服务结构体
struct UserService;

// 实现UserService的方法
impl UserService {
    // 获取所有用户
    #[get("/users")]
    pub fn get_users(req: &GetUsersRequest, db: State<HashMap<u32, User>>) -> status::Ok<serde_json::Value> {
        let db = db.lock().unwrap();
        let mut users = Vec::new();

        // 根据请求参数进行分页处理
        let limit = req.limit.unwrap_or(10);
        let offset = req.offset.unwrap_or(0);
        let mut count = 0;

        for (id, user) in db.iter().skip(offset as usize).take(limit as usize) {
            users.push(serde_json::to_value(user).unwrap());
            count += 1;
        }

        // 返回JSON响应
        serde_json::json!({
            "count": count,
            "users": users,
        })
    }

    // 获取单个用户
    #[get("/users/<id>