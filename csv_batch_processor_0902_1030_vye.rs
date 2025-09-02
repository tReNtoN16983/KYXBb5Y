// csv_batch_processor.rs
// This Rust program uses the Rocket framework to create a web service that processes CSV files in batch.

use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status::NotFound;
use std::fs;
use std::io;
use std::path::Path;
use csv::ReaderBuilder;
use serde::Deserialize;

// Define a structure to hold the request parameters.
#[derive(Deserialize)]
struct RequestParams {
    directory: String,
    output: String,
}

// Define a structure to hold the CSV row data.
#[derive(Deserialize, Serialize)]
struct CsvRow {
    // Define the columns of the CSV row.
    column1: String,
    column2: String,
    // Add more columns as needed.
}

#[get("/process_csv")]
// Endpoint to process CSV files.
async fn process_csv(params: Json<RequestParams>) -> io::Result<&'static str> {
    // Check if the directory exists.
    let dir_path = Path::new(&params.directory);
    if !dir_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"));
    }

    // Iterate over CSV files in the directory.
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("csv") {
            // Process the CSV file.
            process_single_csv(&path).await?;
        }
    }

    Ok("CSV files processed successfully.")
}

// Asynchronously process a single CSV file.
async fn process_single_csv(path: &Path) -> io::Result<()> {
    let mut reader = ReaderBuilder::new().from_path(path)?;
    for result in reader.deserialize() {
        let record: CsvRow = result?;
        // Process the CSV row data.
        // For this example, we'll just print it.
        println!("Processed row: {:?}", record);
    }
    Ok(())
}

#[launch]
// Define the main function to launch the Rocket server.
fn rocket() -> _ {
    rocket::build().mount("/", routes![process_csv])
}