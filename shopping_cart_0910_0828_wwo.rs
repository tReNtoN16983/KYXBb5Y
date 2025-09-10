 * It includes error handling, comments, and follows Rust best practices for maintainability and scalability.
 */

use rocket::get;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;

// Define a product structure with id and name
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Product {
    id: u32,
    name: String,
}

// Define a shopping cart item structure with product and quantity
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CartItem {
    product: Product,
    quantity: u32,
}

// Define a shopping cart structure with a map of products and quantities
#[derive(Default, Serialize, Deserialize)]
struct ShoppingCart {
    items: Mutex<HashMap<u32, CartItem>>,
}

// Define a state for the shopping cart that can be shared across requests
#[macro_export]
macro_rules! shopping_cart {
    () => {
        rocket::State::build::<ShoppingCart>()
            .name("shopping_cart")
            .spawnable()
    };
}

// Add a product to the cart
#[get("/add_to_cart<product_id>&<quantity>")]
fn add_to_cart(product_id: u32, quantity: u32, shopping_cart: &rocket::State<ShoppingCart>) -> rocket::response::status::Custom {
    let mut items = shopping_cart.items.lock().unwrap();
    
    // Check if the product already exists in the cart
    if let Some(item) = items.get_mut(&product_id) {
        item.quantity += quantity;
    } else {
        // Add a new item to the cart
        let product = Product { id: product_id, name: "Product".to_string() }; // Placeholder for product name
        items.insert(product_id, CartItem { product, quantity });
    }
    
    rocket::response::status::Ok::<()>(json!({"message": "Product added to cart"}))
}

// Get the current shopping cart
#[get("/cart")]
fn get_cart(shopping_cart: &rocket::State<ShoppingCart>) -> rocket::serde::json::Json<ShoppingCart> {
    rocket::serde::json::Json(shopping_cart.clone())
}

// Delete a product from the cart
#[get("/remove_from_cart<product_id>")]
fn remove_from_cart(product_id: u32, shopping_cart: &rocket::State<ShoppingCart>) -> rocket::response::status::Custom {
    let mut items = shopping_cart.items.lock().unwrap();
    
    if items.remove(&product_id).is_some() {
        rocket::response::status::Ok::<()>(json!({"message": "Product removed from cart"}))
    } else {
        rocket::response::status::NotFound::<()>(json!({"error": "Product not found in cart"}))
    }
}

// Rocket launch configuration
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_to_cart, get_cart, remove_from_cart])
        .manage(ShoppingCart::default())
        .attach(shopping_cart!())
}
