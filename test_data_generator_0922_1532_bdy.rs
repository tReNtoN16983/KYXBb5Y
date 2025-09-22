use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[macro_use] extern crate serde_derive;

// 定义请求参数结构体
#[derive(Deserialize)]
pub struct TestDataParams {
    pub count: u32,
}

// 定义测试数据结构体
#[derive(Serialize)]
pub struct TestData {
    pub id: u32,
    pub name: String,
    pub email: String,
}

// 实现测试数据生成器
#[get("/test-data")]
#[return_type = "Json<Json<TestData>>"]
fn generate_test_data(params: Json<TestDataParams>) -> Json<Json<Vec<TestData>>> {
    // 根据请求参数生成测试数据
    let data = (1..params.count).map(|i| TestData {
        id: i,
        name: format!("User{}", i),
        email: format!("user{}@example.com", i),
    }).collect();

    // 返回生成的测试数据
    Json(data)
}

// Rocket 启动函数
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate_test_data])
}