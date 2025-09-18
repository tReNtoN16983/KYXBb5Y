// decompression_service.rs
// 使用 Rust 和 Rocket 框架实现压缩文件解压工具

#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::serde::json::Json;
use std::io::{self, Read, Write};
use std::path::Path;
use flate2::read::GzDecoder;
use rocket::response::content::RawHtml;
use rocket::response::status::Created;
use rocket::State;
use rocket::serde::json::serde_json::json;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct UploadFile {
    file: Vec<u8>,
}

#[get("/")]
#[launch]
fn main() -> _ {
    rocket::build()
        .mount("/", routes![home, decompress])
        .manage("./uploads")
}

#[get("/")]
fn home() -> RawHtml<'static> {
    RawHtml(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Decompression Service</title>
        </head>
        <body>
            <h1>Decompression Service</h1>
            <form action="/decompress" method="post" enctype="multipart/form-data">
                <input type="file" name="file" required>
                <input type="submit" value="Decompress">
            </form>
        </body>
        </html>
    