// api_response_formatter.rs
//
// 这是一个使用RUST和ROCKET框架创建的API响应格式化工具。
//
// 它提供了一个简单的API端点，用于格式化响应数据。

#[macro_use] extern crate rocket;

// 定义一个用于API响应的结构体
#[derive(Serialize, Debug, Clone)]
struct ApiResponse<T> {
    // API响应的状态码
    status: usize,
    // API响应的消息
    message: String,
    // API响应的数据
    data: T,
}

// 实现ApiResponse的ToString方法，以便能够格式化为字符串
impl<T: ToString> ApiResponse<T> {
    fn format(&self) -> String {
        format!("{{\'status\': {}, \'message\': \'{}\', \'data\': {} }}", self.status, self.message, self.data.to_string())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![format_response])
}

// 定义一个处理请求的函数
// 这个函数将接收一个简单的整数参数，返回格式化的API响应
#[get("/format/<value>")]
fn format_response(value: i32) -> String {
    // 创建ApiResponse实例
    let api_response = ApiResponse {
        status: 200,
        message: "Success".to_string(),
        data: value,
    };
    
    // 返回格式化后的API响应字符串
    api_response.format()
}
