// integration_test_tool.rs
// 这是一个使用RUST和ROCKET框架创建的集成测试工具程序。

#[macro_use] extern crate rocket;
use rocket::Route;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::outcome::IntoOutcome;
use rocket::serde::{Serialize, Deserialize};
use std::sync::Mutex;

// 定义一个简单的用户模型
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: i32,
    name: String,
}

// 定义用户存储，使用Mutex以确保线程安全
lazy_static! {
    static ref USER_DB: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

// 定义一个请求参数结构体
#[derive(FromForm)]
struct AddUser {
    name: String,
}

// 定义错误处理枚举
#[derive(Debug)]
enum Error {
    UserAlreadyExists,
    InternalServerError,
}

impl<'r> IntoOutcome<'r, Json<User>> for Error {
    fn into_outcome(self) -> rocket::outcome::Outcome<'r, Json<User>, Status> {
        match self {
            Error::UserAlreadyExists =>Outcome::Forward((Status::BadRequest, Json(User { id: 0, name: "User already exists".to_string() }))),
            Error::InternalServerError =>Outcome::Forward((Status::InternalServerError, Json(User { id: 0, name: "Internal server error".to_string() }))),
        }
    }
}

#[post("/add_user", data = "<add_user>")]
fn add_user(add_user: Json<AddUser>, users: &State<Vec<User>>) -> Result<Json<User>, Error> {
    let mut user_db = users.lock().unwrap();
    for user in user_db.iter() {
        if user.name == add_user.name {
            return Err(Error::UserAlreadyExists);
        }
    }
    let new_user = User {
        id: user_db.len() as i32 + 1,
        name: add_user.name.clone(),
    };
    user_db.push(new_user.clone());
    Ok(Json(new_user))
}

// 定义rocket启动函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(USER_DB.clone())
        .mount("/", routes![add_user])
        .attach(
            rocket::Logging::new()
                .level(rocket::config::LogLevel::Debug)
                .format(rocket::fairing::日志::Format::Json)
        )
}
