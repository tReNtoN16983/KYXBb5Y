use rocket::get;
use rocket::post;
use rocket::State;
use rocket::serde::json::Json;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rocket::fairing::AdHoc;

// Define a struct to represent a shopping cart item
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CartItem {
    id: u32,
    quantity: u32,
}

// Define a struct to represent the shopping cart
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
struct ShoppingCart {
    items: HashMap<u32, CartItem>,
}

// Define a global shopping cart using a Mutex to allow safe access from multiple threads
lazy_static! {
    static ref GLOBAL_CART: Mutex<ShoppingCart> = Mutex::new(ShoppingCart::default());
}

#[macro_use] extern crate lazy_static;

#[get("/cart")]
// Get the current state of the shopping cart
fn get_cart() -> Json<ShoppingCart> {
    let cart = GLOBAL_CART.lock().unwrap();
    Json(cart.clone())
}

#[post("/cart/add", format = "json", data = "<item>")]
// Add an item to the shopping cart
fn add_item(item: Json<CartItem>, _state: State<Mutex<ShoppingCart>>) -> String {
    let mut cart = _state.lock().unwrap();
    match cart.items.get_mut(&item.id) {
        Some(cart_item) => {
            cart_item.quantity += item.quantity;
        },
        None => {
            cart.items.insert(item.id, item.clone());
        },
    }
    "Item added to cart".to_string()
}

#[post("/cart/remove", format = "json", data = "<item_id>")]
// Remove an item from the shopping cart by its ID
fn remove_item(item_id: Json<u32>, _state: State<Mutex<ShoppingCart>>) -> String {
    let mut cart = _state.lock().unwrap();
    if cart.items.remove(&item_id).is_some() {
        "Item removed from cart".to_string()
    } else {
        "Item not found in cart".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_attach("Global Cart", |rocket| {
            Ok(rocket.manage(GLOBAL_CART.clone()))
        })).mount("/api", routes![get_cart, add_item, remove_item])
}

// Define the routes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cart() {
        let client = rocket::build().mount("/api", routes![get_cart]).http("localhost").unwrap();
        let response = client.get("/api/cart").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }

    #[test]
    fn test_add_item() {
        let client = rocket::build().mount("/api", routes![add_item]).http("localhost").unwrap();
        let item = Json(CartItem { id: 1, quantity: 2 });
        let response = client.post("/api/cart/add").body(item.to_string()).header("Content-Type", "application/json").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }
}
