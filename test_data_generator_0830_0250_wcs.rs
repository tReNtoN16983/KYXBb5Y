use rocket::get;
use rocket::serde::json::Json;
use serde::Serialize;
use uuid::Uuid;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rocket::response::Content;

// 定义一个结构体来存储用户信息
#[derive(Serialize)]
struct User {
    id: String,
    username: String,
    email: String,
}

// 测试数据生成器服务
#[get("/generate_test_data?<user_count>&<user_name_prefix>&<email_domain>")]
fn generate_test_data(user_count: u32, user_name_prefix: String, email_domain: String) -> Content<&'static str> {
    // 初始化随机数生成器
    let mut rng = rand::thread_rng();
    let mut users = Vec::new();

    // 生成指定数量的用户数据
    for _ in 0..user_count {
        let user_id = Uuid::new_v4().to_string();
        let username = format!("{}",
            rng.sample(Alphanumeric).take(8).map(char::from).collect::<String>()
        );
        let email = format!("{}@{}.{}", username, user_name_prefix, email_domain);

        users.push(User {
            id: user_id,
            username: username.clone(), // 必须克隆，因为User结构体中持有所有权
            email: email,
        });
    }

    // 将用户数据序列化为JSON并返回
    let json = Json::serde(users).to_string();
    Content(json.as_str().into())
}

// 设置ROCKET的主函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![generate_test_data])
}