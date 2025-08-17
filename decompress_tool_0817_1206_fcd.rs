use rocket::get;
use rocket::Route;
use rocket::response::status;
use rocket::response::Stream;
use rocket::Data;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;
use zip::ZipArchive;
use rocket::http::Status;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket::tokio;

#[macro_use]
extern crate rocket;

#[get("/decompress")]
#[launch]
fn main() -> _ {
    rocket::build()
        .mount("/", StaticFiles::from("."))
        .mount("/templates", Rocket::custom()
            .mount("/", StaticFiles::from("./templates"))
            .attach(Template::fairing()))
        .mount("/api", routes![])
        .register("/api", catchers![not_found_handler])
        .register("/", catchers![not_found_handler]);
}

#[get("/api/decompress")]
fn decompress(file: Data<'_>) -> status::Custom<Stream<BufReader<File>>> {
    let file_path = "./tmp.zip";
    let mut file = match File::create(file_path) {
        Ok(f) => f,
        Err(e) => return status::Custom(Status::InternalServerError, format!("Failed to create file: {}", e)),
    };
    let size = file.write_all(file.data()).unwrap_or_else(|_| {
        panic!("Failed to write to file");
    });
    
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return status::Custom(Status::InternalServerError, format!("Failed to open file: {}", e)),
    };
    let file = BufReader::new(file);
    let mut archive = match ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => return status::Custom(Status::InternalServerError, format!("Failed to read zip archive: {}", e)),
    };
    
    let dest_path = "./extracted";
    if let Err(e) = std::fs::create_dir_all(dest_path) {
        return status::Custom(Status::InternalServerError, format!("Failed to create destination directory: {}", e));
    }
    
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(e) => return status::Custom(Status::InternalServerError, format!("Failed to read file in zip: {}", e)),
        };
        
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let mut outfile = match File::create(&Path::new(dest_path).join(&outpath)) {
            Ok(file) => file,
            Err(e) => return status::Custom(Status::InternalServerError, format!("Failed to create output file: {}", e)),
        };        
        
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_) => outfile.write_all(&buffer).unwrap_or_else(|_| {
                panic!("Failed to write to output file");
            }),
            Err(e) => return status::Custom(Status::InternalServerError, format!("Failed to read from zip file: {}", e)),
        }
    }
    
    return status::Custom(Status::Ok, Stream::from(BufReader::new(File::open("./extracted").unwrap())));
}

fn not_found_handler(req: &rocket::Request) -> status::Custom<&'static str> {
    status::Custom(Status::NotFound, "The requested resource was not found")
}
