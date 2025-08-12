use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::str::FromStr;
use std::num::ParseFloatError;
use rocket::response::status;
use rocket::http::Status;

// 定义错误类型
#[derive(Debug, Serialize)]
enum CalculatorError {
    ParseError(ParseFloatError),
    DivisionByZero,
}

// 定义数学操作枚举
#[derive(Serialize)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

// 定义数学运算请求结构体
#[derive(Deserialize)]
struct MathRequest {
    num1: f64,
    num2: f64,
    operation: Operation,
}

// 定义数学运算响应结构体
#[derive(Serialize)]
struct MathResponse {
    result: f64,
    operation: Operation,
    num1: f64,
    num2: f64,
}

// 实现数学运算
impl MathResponse {
    fn new(num1: f64, num2: f64, operation: Operation) -> Result<Self, CalculatorError> {
        let result = match operation {
            Operation::Add => num1 + num2,
            Operation::Subtract => num1 - num2,
            Operation::Multiply => num1 * num2,
            Operation::Divide => {
                if num2 == 0.0 {
                    return Err(CalculatorError::DivisionByZero);
                }
                num1 / num2
            }
            Operation::Modulo => {
                if num2 == 0.0 {
                    return Err(CalculatorError::DivisionByZero);
                }
                num1 % num2 as i64 as f64
            },
        };
        Ok(MathResponse {
            result,
            operation,
            num1,
            num2,
        })
    }
}

// 定义Rocket启动器
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![math_calculate])
        .launch()
        .await
}

// 定义数学运算路由
#[get("/math/<operation>/<num1>/<num2>")]
fn math_calculate(operation: String, num1: f64, num2: f64) -> status::Custom<Json<MathResponse>> {
    match Operation::from_str(&operation) {
        Ok(op) => {
            MathResponse::new(num1, num2, op)
                .map(|res| status::Custom(Status::Ok, Json(res)))
                .map_err(|e| match e {
                    CalculatorError::ParseError(_) => status::Custom(Status::InternalServerError, Json("".to_string())),
                    CalculatorError::DivisionByZero => status::Custom(Status::BadRequest, Json("Division by zero error".to_string())),
                })
        }
        Err(_) => status::Custom(Status::BadRequest, Json("Bad request error".to_string())),
    }
}

// 实现Operation枚举的FromStr trait
impl<'de> FromStr for Operation {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Operation::Add),
            "subtract" => Ok(Operation::Subtract),
            "multiply" => Ok(Operation::Multiply),
            "divide" => Ok(Operation::Divide),
            "modulo" => Ok(Operation::Modulo),
            _ => Err("Invalid operation"),
        }
    }
}