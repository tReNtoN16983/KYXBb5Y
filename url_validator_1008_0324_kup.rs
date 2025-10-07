#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status;
use url::Url;
use std::error::Error;

#[derive(FromForm)]
struct ValidateUrlForm<'r> {
    #[rockets(form = "plain")]
    url: &'r str,
}

#[post("/validate_url", data = "<validate_url_form>")]
fn validate_url(validate_url_form: Form<ValidateUrlForm>) -> Result<String, status::BadRequest<&'static str>> {
    let url = validate_url_form.url;
    
    match Url::parse(url) {
        Ok(_) => Ok(format!("The URL '{}' is valid.", url)).to_json(),
        Err(_) => Err(status::BadRequest(Some("The provided URL is not valid.")))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![validate_url])
}
