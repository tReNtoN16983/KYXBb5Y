use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde_json::Json;
use rocket::http::Status;
use rocket::response::status;
use openssl::sha::{Sha256, Sha512};
use openssl::symm::{encrypt, Cipher};
use openssl::error::ErrorStack;

// 定义请求结构体，用于接收前端传入的数据
#[derive(Deserialize, Serialize, Debug)]
pub struct HashInput {
    pub data: String,
    pub algorithm: String,
}

// 定义响应结构体，用于返回结果
#[derive(Serialize, Debug)]
pub struct HashResponse {
    pub hash_value: String,
}

#[get("/hash")]
// 计算哈希值的API端点
pub fn calculate_hash(input: Json<HashInput>) -> Result<Json<HashResponse>, status::Custom<&'static str>> {
    let algorithm = &input.algorithm;
    let data = input.data.as_bytes();

    // 根据算法选择哈希函数
    let hash_value = match algorithm.as_ref() {
        "sha256" => calculate_sha256(data),
        "sha512" => calculate_sha512(data),
        _ => return Err(status::Custom(Status::BadRequest, "Unsupported algorithm")),
    };

    // 返回哈希值
    Ok(Json(HashResponse { hash_value }))
}

// 使用SHA256算法计算哈希值
fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finish();
    // 将哈希值格式化为十六进制字符串
    hex::encode(result)
}

// 使用SHA512算法计算哈希值
fn calculate_sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finish();
    // 将哈希值格式化为十六进制字符串
    hex::encode(result)
}

fn main() {
    rocket::build()
        .mount("/", routes![calculate_hash])
        .launch();
}
