 * and ensures maintainability and extensibility of the code.
 */

// Import the necessary Rocket components and other modules
#[macro_use] extern crate rocket;

// Define a module for request handlers
mod request_handlers;

// Entry point of the application
#[launch]
fn rocket() -> _ {
    // Mount the request handlers
    rocket::build().mount("/api", request_handlers::routes())
}

// Module containing all the request handlers
mod request_handlers {
    use rocket::get;
    use rocket::http::Status;
    use rocket::response::status;
    use rocket::serde::json::Json;
    
    // Define a structure to represent a response
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Response {
        message: String,
    }

    // A simple GET endpoint that returns a message
    #[get("/hello")]
    async fn hello() -> Result<Json<Response>, status::Custom<Status>> {
        // Return a success response
        Ok(Json(Response { message: "Hello, world!".to_string() }))
    }

    // Define the routes for the module
    pub fn routes() -> rocket::Route {
        rocket::Route::new(rocket::http::Method::Get, "/hello", hello)
    }
}
