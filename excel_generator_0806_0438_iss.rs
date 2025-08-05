 * Usage:
 * 1. Start the server with `cargo run`.
 * 2. Send a POST request to `/generate` with a JSON payload that includes your data.
 * 3. The server will respond with a link to download the generated Excel file.
 */

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::Redirect;
use rocket::http::Status;
use rocket::State;
use rocket::response::Responder;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use serde::Serialize;
use excelwriter::ExcelFile;
use rocket::form::Form;

// Define a struct to represent the data to be written to the Excel file.
#[derive(Serialize, Deserialize)]
pub struct ExcelData {
    sheets: Vec<SheetData>,
}

// Define a struct to represent a single sheet in the Excel file.
#[derive(Serialize, Deserialize)]
pub struct SheetData {
    name: String,
    data: Vec<Vec<String>>,
}

// Define a struct to hold the generated Excel file's path.
#[derive(Responder)]
struct DownloadExcelFile(PathBuf);

impl<'r> Responder<'r, 'static> for DownloadExcelFile {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let path_str = self.0.to_str().unwrap();
        let mut response = rocket::Response::build();
        response.set_header(rocket::http::ContentType::Plain);
        response.set_sized_body(path_str.len(), || {
            let mut body = rocket::response::Body::from_string(path_str).into_inner();
            std::io::copy(&mut body, &mut std::io::stdout())
        }).ok();
        Ok(response)
    }
}

#[post("/generate", format = "json", data = "<excel_data>")]
fn generate_excel_file(excel_data: Json<ExcelData>) -> Result<DownloadExcelFile, Status> {
    // Create a new Excel file.
    let mut file = ExcelFile::new().map_err(|e| {
        eprintln!("Failed to create Excel file: {}", e);
        Status::InternalServerError
    })?;

    // Iterate over the sheets and write data to each.
    for sheet in excel_data.sheets {
        let mut sheet_writer = file.add_sheet(&sheet.name).map_err(|e| {
            eprintln!("Failed to add sheet: {}", e);
            Status::InternalServerError
        })?;

        for row in sheet.data {
            sheet_writer.write_row(&row).map_err(|e| {
                eprintln!("Failed to write row: {}", e);
                Status::InternalServerError
            })?;
        }
    }

    // Save the Excel file to a temporary directory.
    let output_path = PathBuf::from("./output").join("generated_excel.xlsx");
    fs::write(&output_path, file).map_err(|e| {
        eprintln!("Failed to write Excel file: {}", e);
        Status::InternalServerError
    })?;

    // Return a response with a link to download the generated Excel file.
    Ok(DownloadExcelFile(output_path))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![generate_excel_file])
        .manage(PathBuf::from("./output").canonicalize().expect("Failed to canonicalize path")
        )
}