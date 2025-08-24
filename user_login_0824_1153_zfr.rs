use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::Route;
use rocket::response::status;
use std::sync::Mutex;
use std::collections::HashMap;

// Define a user struct with username and password
#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
}

// Define a form data structure for handling login data
#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}

// Define a database entry struct for storing users
struct UserDBEntry {
    username: String,
    password: String,
}

// A simple in-memory database to store user data
struct InMemoryUserDB {
    users: Mutex<HashMap<String, UserDBEntry>>,
}

impl InMemoryUserDB {
    // Create a new in-memory database
    fn new() -> Self {
        InMemoryUserDB {
            users: Mutex::new(HashMap::new()),
        }
    }

    // Add a new user to the database
    fn add_user(&self, username: &str, password: &str) {
        let mut users = self.users.lock().unwrap();
        users.insert(username.to_string(), UserDBEntry {
            username: username.to_string(),
            password: password.to_string(),
        });
    }
}

// Define the main application struct
#[rocket::main]
async fn main() {
    // Initialize the in-memory database
    let user_db = InMemoryUserDB::new();
    user_db.add_user("admin", "password123");

    // Define routes for the user login system
    rocket::build()
        .manage(user_db)
        .mount("/", routes![login_get, login_post])
        .launch()
        .await
        .expect("Server launch failed");
}

// Define the GET route for the login page
#[get("/login")]
fn login_get() -> String {
    ""
}

// Define the POST route for handling login requests
#[post("/login", data = "<login>")]
async fn login_post(login: Json<Login>, user_db: &State<InMemoryUserDB>) -> status::Custom<&'static str> {
    let db_users = user_db.users.lock().unwrap();
    if let Some(user) = db_users.get(&login.username) {
        if user.password == login.password {
            return status::Custom(Status::Ok, "Login successful");
        } else {
            return status::Custom(Status::Unauthorized, "Invalid username or password");
        }
    } else {
        return status::Custom(Status::NotFound, "User not found");
    }
}
