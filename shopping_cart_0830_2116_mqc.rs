// shopping_cart.rs
// 一个简单的购物车功能的实现

use rocket::get;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

// 定义购物车项目结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CartItem {
    id: i32,
    name: String,
    price: f64,
}

// 购物车结构，包含一个HashMap来存储购物车项目
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ShoppingCart {
    items: HashMap<i32, CartItem>,
}

// 购物车服务
#[derive(Serialize, Deserialize, Debug)]
struct ShoppingCartService {
    cart: ShoppingCart,
}

#[get("/cart")]
// 获取购物车内容
fn get_cart(cart_service: &State<ShoppingCartService>) -> Json<ShoppingCart> {
    Json(cart_service.cart.clone())
}

#[post("/cart", format = "json", data = "<item>")]
// 添加项目到购物车
fn add_to_cart(item: Json<CartItem>, cart_service: &State<ShoppingCartService>) -> Json<ShoppingCart> {
    cart_service.cart.items.insert(item.id, item.into_inner().clone());
    Json(cart_service.cart.clone())
}

#[post("/cart/remove", format = "json", data = "<item_id>")]
// 从购物车移除项目
fn remove_from_cart(item_id: Json<i32>, cart_service: &State<ShoppingCartService>) -> Json<ShoppingCart> {
    cart_service.cart.items.remove(&item_id.into_inner());
    Json(cart_service.cart.clone())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(ShoppingCartService {
            cart: ShoppingCart::default(),
        })
        .mount("/", routes![get_cart, add_to_cart, remove_from_cart])
}
