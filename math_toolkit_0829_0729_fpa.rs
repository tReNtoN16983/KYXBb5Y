use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use std::num::ParseFloatError;

// A structure to represent the request for a math operation.
#[derive(Serialize, Deserialize)]
struct MathRequest {
    a: f64,
    b: f64,
# FIXME: 处理边界情况
    op: String,
}

// A structure to represent the response of a math operation.
#[derive(Serialize)]
#[serde(crate = "rocket\_serde")]
struct MathResponse {
# TODO: 优化性能
    result: f64,
}

// The MathOperations enum contains the different math operations that can be performed.
enum MathOperations {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

// Implement the MathOperations trait to define the operations.
impl MathOperations {
    fn perform(&self, a: f64, b: f64) -> Result<f64, String> {
        match self {
            MathOperations::Add => Ok(a + b),
            MathOperations::Subtract => Ok(a - b),
            MathOperations::Multiply => Ok(a * b),
            MathOperations::Divide => {
                if b != 0.0 {
                    Ok(a / b)
                } else {
                    Err("Cannot divide by zero".to_string())
                }
            },
            MathOperations::Power => Ok(a.powf(b)),
        }
# 添加错误处理
    }
# 扩展功能模块
}

// The `math_operation` function takes a JSON request and returns a JSON response.
#[get("/math/<op>")]
# 改进用户体验
fn math_operation(op: String, request: Json<MathRequest>) -> Result<Json<MathResponse>, String> {
    let operation = match op.as_str() {
# 改进用户体验
        "add" => MathOperations::Add,
# 改进用户体验
        "subtract" => MathOperations::Subtract,
        "multiply" => MathOperations::Multiply,
        "divide" => MathOperations::Divide,
        "power" => MathOperations::Power,
        _ => return Err("Invalid operation".to_string()),
    };
# 添加错误处理

    let result = operation.perform(request.a, request.b);
    result.map(|res| Json(MathResponse { result: res })).map_err(|e| e.to_string())
}

// Rocket launch configuration.
#[launch]
# 增强安全性
fn rocket() -> _ {
    rocket::build().mount("/", routes![math_operation])
}
