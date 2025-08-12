use rocket::get;
use rocket::post;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::Request;
use rocket::Outcome;
use rocket::http::Status;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// 定义订单模型
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Order {
    id: u32,
    product_id: u32,
    quantity: u32,
    status: String,
}

// 定义错误类型
#[derive(Debug)]
enum OrderError {
    InvalidOrder,
    InternalServerError,
}

// 实现错误转换为HTTP状态码
impl From<OrderError> for status::Custom<'static> {
    fn from(err: OrderError) -> Self {
        match err {
            OrderError::InvalidOrder => status::Custom(Status::BadRequest, "Invalid order"),
            OrderError::InternalServerError => status::Custom(Status::InternalServerError, "Internal Server Error"),
        }
    }
}

// 订单处理业务逻辑
#[post("/process_order", format = "json", data = "<order>")]
fn process_order(order: Json<Order>) -> Result<status::Created<'static>, status::Custom<'static>> {
    if order.quantity == 0 {
        return Err(OrderError::InvalidOrder.into());
    }

    // 模拟订单处理逻辑
    order.status = "processed".to_string();

    Ok(status::Created::new("/orders/".to_owned(), order.into_inner()))
}

// 订单列表
lazy_static! {
    static ref ORDERS: Mutex<HashMap<u32, Order>> = Mutex::new(HashMap::new());
}

// 获取所有订单
#[get("/orders")]
fn get_orders() -> &'static str {
    // 模拟从数据库获取所有订单
    let orders = ORDERS.lock().unwrap();
    "List of all orders"
}

// 启动Rocket应用
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![process_order, get_orders])
}
