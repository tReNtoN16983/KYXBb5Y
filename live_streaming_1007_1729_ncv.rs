use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rocket::response::status;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

// Define a struct to hold the state of the live stream.
#[derive(Debug, Clone)]
struct LiveStream {
    // Simulate stream metadata
    title: String,
    view_count: u32,
}

// Define a global state to store live streams.
#[macro_export]
macro_rules! live_streams {
    () => {
        Arc::new(Mutex::new(HashMap::new()))
    };
}

#[get("/streams")]
// List all live streams.
fn list_streams(streams: &State<Arc<Mutex<HashMap<String, LiveStream>>>) -> Json<Vec<LiveStream>> {
    let streams = streams.lock().unwrap();
    let mut result = Vec::new();
    for stream in streams.values() {
        result.push(stream.clone());
    }
    Json(result)
}

#[get("/streams/<id>")]
// Get a specific live stream by ID.
fn get_stream(id: String, streams: &State<Arc<Mutex<HashMap<String, LiveStream>>>) -> status::Custom<String> {
    let streams = streams.lock().unwrap();
    match streams.get(&id) {
        Some(stream) => Json(stream.clone()).into(),
        None => status::Custom(Status::NotFound, "Stream not found".to_string()).into(),
    }
}

#[launch]
// Start the Rocket server.
fn rocket() -> _ {
    rocket::build()
        .manage(live_streams!())
        .mount("/", routes![list_streams, get_stream])
}
