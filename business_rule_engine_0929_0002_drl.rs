use rocket::get;
use rocket::Route;
use rocket::serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::result::Result;
use rocket::http::Status;

// 定义业务规则执行结果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleExecutionResult {
    pub success: bool,
    pub message: String,
    pub data: Option<HashMap<String, String>>,
}

// 定义业务规则接口
pub trait BusinessRule {
    fn execute(&self) -> Result<RuleExecutionResult, RuleExecutionError>;
}

// 定义业务规则错误
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleExecutionError {
    pub code: i32,
    pub message: String,
}

// 实现一个示例业务规则
pub struct SampleBusinessRule;

impl BusinessRule for SampleBusinessRule {
    fn execute(&self) -> Result<RuleExecutionResult, RuleExecutionError> {
        // 这里添加业务逻辑
        // ...
        
        // 模拟成功执行结果
        Ok(RuleExecutionResult {
            success: true,
            message: "Rule executed successfully".to_string(),
            data: Some(HashMap::from(["key".to_string(), "value".to_string()])),
        })
    }
}

// 定义Rocket启动器结构
#[derive(Debug)]
pub struct RuleEngine;

#[rocket::main]
impl<'r> RuleEngine {
    #[get("/execute")]
    pub async fn execute_rule() -> Result<RuleExecutionResult, RuleExecutionError> {
        let rule = SampleBusinessRule {};
        let result = rule.execute();

        match result {
            Ok(execution_result) => Ok(execution_result),
            Err(e) => Err(e),
        }
    }
}

// 定义Rocket路由
pub fn routes() -> Vec<Route> {
    routes![RuleEngine::execute_rule]
}

fn main() {
    rocket::build()
        .mount("/", routes())
        .launch();
}
