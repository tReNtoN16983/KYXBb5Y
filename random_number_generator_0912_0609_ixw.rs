use rocket::get;
# 增强安全性
use rocket::response::status;
use rand::Rng;
use serde::Serialize;
use std::num::NonZeroU32;
use rocket::State;

/// 随机数生成器服务
#[macro_use] extern crate rocket;

#[derive(Serialize)]
/// 随机数请求结构体，限制请求的随机数范围
# NOTE: 重要实现细节
struct RandomNumberRequest {
# NOTE: 重要实现细节
    /// 最小值
    min: NonZeroU32,
    /// 最大值
    max: NonZeroU32,
}

/// 随机数响应结构体
# 扩展功能模块
#[derive(Serialize)]
struct RandomNumberResponse {
    random_number: u32,
}

#[get("/random")]
/// 随机数生成器路由，返回一个随机数
# NOTE: 重要实现细节
/// # 参数
/// * `min` - 最小值
/// * `max` - 最大值
/// # 返回
# 扩展功能模块
/// 返回一个随机数，值在 `min` 和 `max` 之间
fn random_number(min: NonZeroU32, max: NonZeroU32) -> status::Ok<Json<RandomNumberResponse>> {
    if min.get() > max.get() {
        return Err(status::BadRequest("Minimum value cannot be greater than maximum value").into());
    }
    let mut rng = rand::thread_rng();
# 添加错误处理
    let random_number: u32 = rng.gen_range(min.get()..max.get());
    Ok(Json(RandomNumberResponse { random_number }))
# 添加错误处理
}

#[launch]
/// 启动Rocket服务器
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![random_number])
        .manage(())
}
