use rocket::get;
use rocket::serde::json::Json;
use rocket::response::Status;
use rocket::serde::json::serde_json::json;
use rocket::http::Status as HttpStatus;
use rocket::State;
# NOTE: 重要实现细节
use std::sync::Mutex;
use std::collections::HashMap;

// 引入Rocket的API结构体
# 添加错误处理
#[macro_use] extern crate rocket;

// 定义用户数据结构
# 增强安全性
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// 引入rocket::State来管理全局状态
#[database("users_db")]
struct DbUser(Mutex<HashMap<i32, User>>);

// 创建一个新的用户
#[post("/users", data = "<user>")]
fn create_user(user: Json<User>, db: &State<DbUser>) -> Status {
    let mut users = db.0.lock().unwrap();
# FIXME: 处理边界情况
    users.insert(user.id, user.into_inner().clone());
# 增强安全性
    Status::Created
# 改进用户体验
}
# TODO: 优化性能

// 获取单个用户信息
# 增强安全性
#[get("/users/<id>")]
fn get_user(id: i32, db: &State<DbUser>) -> Result<Json<User>, Status> {
    let users = db.0.lock().unwrap();
    match users.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(Status::NotFound),
# 优化算法效率
    }
}

// 更新用户信息
#[put("/users/<id>", data = "<user>")]
fn update_user(id: i32, user: Json<User>, db: &State<DbUser>) -> Result<Status, Status> {
    let mut users = db.0.lock().unwrap();
    match users.get_mut(&id) {
        Some(user_data) => {
            user_data.name = user.into_inner().name;
            user_data.email = user.into_inner().email;
            Ok(Status::Ok)
        },
        None => Err(Status::NotFound),
    }
}

// 删除用户
#[delete("/users/<id>")]
fn delete_user(id: i32, db: &State<DbUser>) -> Result<Status, Status> {
    let mut users = db.0.lock().unwrap();
# TODO: 优化性能
    match users.remove(&id) {
        Some(_) => Ok(Status::Ok),
        None => Err(Status::NotFound),
    }
}

#[launch]
fn rocket() -> _ {
    // 设置Rocket配置
    rocket::build()
        .attach(DbUser::init())
        .mount("/api", routes![create_user, get_user, update_user, delete_user])
}
