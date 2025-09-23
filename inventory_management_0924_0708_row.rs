use rocket::serde::{Serialize, Deserialize};
    use rocket::State;
    use rocket::http::Status;
    use std::sync::Mutex;

    // 库存项结构体定义
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct InventoryItem {
        id: u32,
        name: String,
        quantity: u32,
        price: f64,
    }

    // 库存管理系统
    #[derive(Clone)]
    pub struct InventoryManager {
        items: Mutex<Vec<InventoryItem>>,
    }

    // 初始化库存管理系统
    impl InventoryManager {
        pub fn new() -> Self {
            InventoryManager {
                items: Mutex::new(vec![
                    InventoryItem {
                        id: 1,
                        name: "Item 1".to_string(),
                        quantity: 10,
                        price: 9.99,
                    },
                    InventoryItem {
                        id: 2,
                        name: "Item 2".to_string(),
                        quantity: 20,
                        price: 19.99,
                    },
                ]),
            }
        }

        pub fn add_item(&self, item: InventoryItem) -> Result<(), String> {
            let mut items = self.items.lock().map_err(|_| "Failed to lock items".to_string())?;
            items.push(item);
            Ok(())
        }

        pub fn get_items(&self) -> Vec<InventoryItem> {
            let items = self.items.lock().unwrap();
            items.clone()
        }

        pub fn get_item(&self, id: u32) -> Result<InventoryItem, String> {
            let items = self.items.lock().unwrap();
            items.iter().find(|x| x.id == id).cloned().ok_or_else(|| format!("Item with id {} not found", id))
        }

        pub fn update_item(&self, id: u32, quantity: u32, price: f64) -> Result<(), String> {
            let mut items = self.items.lock().map_err(|_| "Failed to lock items".to_string())?;
            if let Some(item) = items.iter_mut().find(|x| x.id == id) {
                item.quantity = quantity;
                item.price = price;
                Ok(())
            } else {
                Err(format!("Item with id {} not found", id))
            }
        }

        pub fn delete_item(&self, id: u32) -> Result<(), String> {
            let mut items = self.items.lock().map_err(|_| "Failed to lock items".to_string())?;
            items.retain(|x| x.id != id);
            Ok(())
        }
    }

    #[macro_use]
    extern crate rocket;

    #[launch]
    fn rocket() -> _ {
        rocket::build()
            .mount(