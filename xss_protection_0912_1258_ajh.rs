#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use htmlescape::encode_minimal;

// Define a structure to hold the input data
#[derive(Serialize, Deserialize)]
struct HtmlInput {
    data: String,
}

// Define a state for storing the HTML escaper
#[derive(Default)]
struct HtmlEscaper;

// Define a fairing to handle HTML escaping for all responses
#[rocket::async_trait]
impl<'r> rocket::fairing::Fairing<'r> for HtmlEscaper {
    fn on_response<'o>(&self, _request: &'o rocket::Request<'_>, response: &mut rocket::response::Response<'o>) {
        if let rocket::response::ContentType::HTML = response.content_type() {
            if let Ok(body) = response.body_mut().as_bytes_mut() {
                let escaped_body = encode_minimal(String::from_utf8_lossy(body));
                response.set_body(escaped_body);
            }
        }
    }
}

#[rocket::main]
async fn main() {
    let html_escaper = HtmlEscaper::default();
    rocket::build()
        .mount("/", routes![handle_input])
        .attach(html_escaper)
        .launch()
        .await
        .expect("Rocket has encountered an error");
}

// Define a route to handle input, escape it, and return the result
#[post("/xss_protect", format = "json", data = "<input>")]
async fn handle_input(input: Json<HtmlInput>) -> status::Custom<&'static str> {
    let sanitized_data = encode_minimal(&input.data);
    
    // You can use the sanitized data to perform actions or store it in the database
    // For this example, we simply return the sanitized data
    Ok(status::Custom(Status::Ok, sanitized_data))
}
