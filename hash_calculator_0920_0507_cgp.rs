 * accepts a string input and returns the corresponding hash value.
 */

use rocket::get;
use rocket::serde::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use sha2::{Sha256, Digest};
use std::io::Cursor;
use rocket::response::status;

#[macro_use]
extern crate rocket;

#[get("/hash")]
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", StaticFiles::from("./static"))
        .mount("/templates", rocket::routes![index])
        .register("errors", rocket::routes![not_found])
}

#[get("/hash/<text>")]
fn hash(input: String) -> Result<Json<String>, status::NotFound<&'static str>> {
    let hash_result = match calculate_hash(&input) {
        Ok(hash) => hash,
        Err(_) => return Err(status::NotFound("Failed to calculate hash".to_string())),
    };
    Ok(Json(hash_result))
}

#[get("/templates/index.html")]
fn index() -> Template {
    Template::render("index", &rocket::State::fetch())
}

fn calculate_hash(input: &str) -> Result<String, &'static str> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn not_found() -> status::NotFound<&'static str> {
    status::NotFound("Resource Not Found")
}