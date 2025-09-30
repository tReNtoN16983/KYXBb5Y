// realtime_data_stream_processor.rs
// This Rust program uses the Rocket framework to create a
// real-time data stream processor.

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::tokio;
use rocket::tokio::sync::mpsc;
use rocket::tokio::stream::{StreamExt, StreamMap};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::response::status;
use rocket::outcome::IntoOutcome;
use rocket::request::{Ours, Outcome, Request};
use rocket::response::Response;
use rocket::http::Status;
use std::collections::HashMap;
use std::sync::Arc;
use rocket::Config;

// Define the structure for the incoming data stream message.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct StreamMessage {
    timestamp: u64,
    data: String,
}

// Define a fairing to handle real-time data stream connections.
#[rocket::async_trait]
impl<'r> rocket::fairing::Fairing<'r> for StreamFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Realtime Data Stream Fairing",
            kind: rocket::fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: rocket::Rocket<'r>) -> rocket::Rocket<'r> {
        let (tx, mut rx) = mpsc::channel(100); // Create a channel with a buffer size of 100.
        let stream_map = StreamMap::new();
        let stream_map = Arc::new(stream_map);

        rocket.manage(stream_map).manage(tx)
    }
}

// Define the fairing for handling real-time data stream.
struct StreamFairing;

// Define the route for real-time data stream.
#[post("/stream", format = "json", data = "<stream_message>")]
async fn stream_data(
    stream_message: StreamMessage,
    stream_map: &State<Arc<StreamMap<u64, mpsc::Sender<StreamMessage>>>>,
) -> Result<status::NoContent, status::InternalServerError> {
    // Find an existing sender for the stream or create a new one if it doesn't exist.
    let sender = stream_map.entry(stream_message.timestamp).or_insert_with(|| {
        let (tx, rx) = mpsc::channel(100);
        mpsc::Sender::new(tx)
    });

    // Try to send the message to the stream.
    if sender.send(stream_message).await.is_err() {
        // If sending fails, return an internal server error.
        return Err(status::InternalServerError);
    }
    // Return a no content response if the message is sent successfully.
    Ok(status::NoContent)
}

// Define the route to subscribe to the real-time data stream.
#[get("/subscribe/<timestamp>")]
async fn subscribe(
    timestamp: u64,
    stream_map: &State<Arc<StreamMap<u64, mpsc::Sender<StreamMessage>>>>,
) -> Result<Ours<'_>, status::InternalServerError> {
    // Find the sender associated with the given timestamp.
    let sender = stream_map.get(&timestamp)
        .ok_or(status::InternalServerError)?;
    // If a sender is found, return a streaming response.
    Ok(stream! {
        for message in sender {
            let message = message.await.unwrap_or_default();
            yield format!("data: {}\
\
", serde_json::to_string(&message).unwrap());
        }
    })
}

// Define the main function to launch the Rocket server.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("sync_stream", move |rocket| {
            Ok(rocket)
        })).mount("/", routes![stream_data, subscribe])
}
