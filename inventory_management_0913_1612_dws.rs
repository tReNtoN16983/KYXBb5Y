use rocket::get;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use std::sync::Mutex;
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

// 定义一个库存项结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct InventoryItem {
    pub id: i32,
    pub name: String,
    pub quantity: i32,
}

// 应用状态：用于存储库存项的全局状态
#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    pub items: Mutex<HashMap<i32, InventoryItem>>,
}

// 获取库存项列表的端点
#[get("/items")]
fn get_items(state: &State<AppState>) -> Result<serde_json::Value, Status> {
    let items = state.items.lock().unwrap();
    let items_vec = items.values().cloned().collect::<Vec<InventoryItem>>();
    Ok(serde_json::json!(items_vec))
}

// 添加库存项的端点
#[post("/items", format = "json", data = "<item>")]
fn add_item(item: InventoryItem, state: &State<AppState>) -> Result<serde_json::Value, Status> {
    let mut items = state.items.lock().unwrap();
    if items.contains_key(&item.id) {
        return Err(Status::new(Status::Code::Conflict, "Item already exists"));
    }
    items.insert(item.id, item.clone());
    Ok(serde_json::json!(item))
}

// 更新库存项的端点
#[put("/items/<id>", format = "json", data = "<item>")]
fn update_item(id: i32, item: InventoryItem, state: &State<AppState>) -> Result<serde_json::Value, Status> {
    let mut items = state.items.lock().unwrap();
    if !items.contains_key(&id) {
        return Err(Status::new(Status::Code::NotFound, "Item not found"));
    }
    items.insert(id, item.clone());
    Ok(serde_json::json!(item))
}

// 删除库存项的端点
#[delete("/items/<id>")]
fn delete_item(id: i32, state: &State<AppState>) -> Result<(), Status> {
    let mut items = state.items.lock().unwrap();
    if !items.contains_key(&id) {
        return Err(Status::new(Status::Code::NotFound, "Item not found"));
    }
    items.remove(&id);
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState {
            items: Mutex::new(HashMap::new()),
        }).mount("/", routes![get_items, add_item, update_item, delete_item])
}
