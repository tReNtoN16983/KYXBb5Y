use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use std::result::Result;

// 定义一个结构体来表示数学计算工具集
#[derive(Debug)]
struct MathTool {
}

impl MathTool {
    // 计算两个数的加法
    #[allow(dead_code)]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // 计算两个数的减法
    #[allow(dead_code)]
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // 计算两个数的乘法
    #[allow(dead_code)]
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // 计算两个数的除法，并处理除以零的错误
    #[allow(dead_code)]
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}

// 定义一个错误类型来表示数学计算错误
#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    InvalidOperation,
}

// 定义一个路由组来处理数学计算请求
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[derive(Serialize)]
struct MathResponse<T> {
    result: T,
    error: Option<String>,
}

#[get("/add/<a>/<b>")]
fn add(a: i32, b: i32) -> status::Custom<Json<MathResponse<i32>> {
    let result = MathTool::add(a, b);
    status::Custom(Status::Ok, Json(MathResponse { result, error: None }))
}

#[get("/subtract/<a>/<b>")]
fn subtract(a: i32, b: i32) -> status::Custom<Json<MathResponse<i32>>> {
    let result = MathTool::subtract(a, b);
    status::Custom(Status::Ok, Json(MathResponse { result, error: None }))
}

#[get("/multiply/<a>/<b>")]
fn multiply(a: i32, b: i32) -> status::Custom<Json<MathResponse<i32>>> {
    let result = MathTool::multiply(a, b);
    status::Custom(Status::Ok, Json(MathResponse { result, error: None }))
}

#[get("/divide/<a>/<b>")]
fn divide(a: i32, b: i32) -> status::Custom<Json<MathResponse<i32>>> {
    match MathTool::divide(a, b) {
        Ok(result) => status::Custom(Status::Ok, Json(MathResponse { result, error: None })),
        Err(e) => status::Custom(Status::BadRequest, Json(MathResponse { result: 0, error: Some(e) })),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/math", routes![add, subtract, multiply, divide])
}