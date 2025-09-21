use rocket::serde::{Deserialize, Serialize};
    use rocket::State;
# NOTE: 重要实现细节
    use rocket::serde::json::Json;
# NOTE: 重要实现细节
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    // Define a User struct that holds the user's permissions
    #[derive(Debug, Serialize, Deserialize, Clone)]
    struct User {
        id: i32,
        username: String,
        permissions: Vec<String>,
    }

    // Define a struct for the UsersManager which will manage the users and their permissions
    struct UsersManager {
        users: Arc<Mutex<HashMap<i32, User>>>,
    }

    #[macro_use] extern crate rocket;

    // Define the rocket launch configuration
    #[launch]
    fn rocket() -> _ {
        rocket::build()
            .mount(