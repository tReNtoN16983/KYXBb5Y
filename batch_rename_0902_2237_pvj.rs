use rocket::get;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::State;
use rocket::fs::NamedFile;
use std::path::Path;
use std::fs;
use std::io;
use regex::Regex;
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct RenameForm {
    dir: String,
    pattern: String,
    replacement: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RenameResult {
    success: bool,
    message: String,
}

#[get("/rename")]
fn rename_files(form: Form<RenameForm>,
                    rocket_db: &State<Regex>) -> Json<RenameResult> {
    let RenameForm { dir, pattern, replacement } = form.into_inner();
    let db_pattern = rocket_db.as_str();

    if !Path::new(&dir).is_dir() {
        return Json(RenameResult {
            success: false,
            message: "Directory does not exist.".to_string(),
        });
    }

    // Compile the regex pattern
    let regex = match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(_) => return Json(RenameResult {
            success: false,
            message: "Invalid regex pattern.".to_string(),
        }),
    };

    // Prepare a map to track old and new filenames
    let mut rename_map: HashMap<String, String> = HashMap::new();

    // Iterate over all files in the directory
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to iterate directory entries");
        let path = entry.path();
        let file_name = path.file_name().expect("Failed to get filename").to_str().unwrap().to_string();

        // Check if the filename matches the regex pattern
        if regex.is_match(&file_name) {
            let new_name = regex.replace_all(&file_name, &replacement).to_string();
            // Add to the map
            rename_map.insert(file_name, new_name);
        }
    }

    // Perform the renaming
    for (old_name, new_name) in &rename_map {
        let path = Path::new(&dir).join(old_name);
        let new_path = Path::new(&dir).join(new_name);
        if fs::rename(&path, &new_path).is_err() {
            return Json(RenameResult {
                success: false,
                message: format!("Failed to rename file: {} to {}", old_name, new_name),
            });
        }
    }

    Json(RenameResult {
        success: true,
        message: "Files renamed successfully.".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![rename_files])
        .manage(Regex::new(r"^[^/]+").unwrap())
}

/**
 * This is a simple web application using the Rocket framework that provides a batch file renaming tool.
 * It takes a directory, a regex pattern to match filenames, and a replacement string to create new filenames.
 * It returns a JSON response indicating the success or failure of the operation.
 */