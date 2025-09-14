#[macro_use]
extern crate rocket;

use rocket::get;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::response::status::NotFound;
use rocket::response::status::BadRequest;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::error::Error;

// Define a structure to hold the processed data
#[derive(serde::Serialize, serde::Deserialize)]
struct CsvRow {
    // Define fields according to your CSV structure
    field1: String,
    field2: String,
    // ...
}

#[get("/process_csv")]
fn process_csv() -> Result<Json<()>, BadRequest<&'static str>> {
    // Read the input CSV file
    let input_file_path = "input.csv";
    let output_file_path = "output.csv";
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_file_path)
        .map_err(|e| BadRequest::new(e.description()))?;

    // Read the CSV file into a Vec<CsvRow>
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| BadRequest::new(e.description()))?;
        rows.push(CsvRow {
            field1: record[0].clone(),
            field2: record[1].clone(),
            // ...
        });
    }

    // Process each row (for example, modify the content)
    for row in rows.iter_mut() {
        // Add your processing logic here
    }

    // Write the processed data to the output CSV file
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_file_path)
        .map_err(|e| BadRequest::new(e.description()))?;
    wtr.write_record(&[