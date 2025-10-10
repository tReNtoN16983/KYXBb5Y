This server provides a simple RPC interface for making remote procedure calls.
*/
use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

// Define a simple RPC request structure with function name and arguments.
#[derive(Serialize, Deserialize)]
pub struct RpcRequest {
    pub function_name: String,
    pub arguments: HashMap<String, String>,
}

// Define a simple RPC response structure.
#[derive(Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: String,
    pub error: Option<String>,
}

// Function registry to map function names to their implementations.
lazy_static::lazy_static! {
    static ref FUNCTIONS: Mutex<HashMap<String, Box<dyn Fn(HashMap<String, String>) -> Result<String, String>>>> = Mutex::new(HashMap::new());
}

// Function to register functions in the registry.
pub fn register_function(function_name: &str, function: impl Fn(HashMap<String, String>) -> Result<String, String> + 'static) {
    let mut functions = FUNCTIONS.lock().unwrap();
    functions.insert(function_name.to_string(), Box::new(function));
}

// RPC handler function.
#[get("/rpc")]
fn rpc_handler(req: Json<RpcRequest>) -> Result<Json<RpcResponse>, Status> {
    // Look up the function in the registry.
    let mut functions = FUNCTIONS.lock().unwrap();
    if let Some(func) = functions.get(&req.function_name) {
        // Call the function with the provided arguments.
        match func(req.arguments.clone()) {
            Ok(result) => Ok(Json(RpcResponse {
                result: result,
                error: None,
            })),
            Err(error) => Ok(Json(RpcResponse {
                result: String::new(),
                error: Some(error),
            })),
        }
    } else {
        // Return an error if the function is not found.
        Err(Status::NotFound)
    }
}

// Main function to setup the Rocket application.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![rpc_handler])
        // Register functions here.
        // register_function("sum", sum);
}

// Example function that can be registered to the RPC server.
pub fn sum(args: HashMap<String, String>) -> Result<String, String> {
    // Simple sum implementation for demonstration purposes.
    if let (Some(a), Some(b)) = (args.get("a"), args.get("b")) {
        match a.parse::<i32>().zip(b.parse::<i32>()) {
            Ok((a, b)) => Ok((a + b).to_string()),
            Err(_) => Err("Invalid input: expected integers".to_string()),
        }
    } else {
        Err("Missing arguments".to_string())
    }
}

// Add necessary dependencies in Cargo.toml:
// [dependencies]
// rocket = "0.5"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// lazy_static = "1.4"
// 