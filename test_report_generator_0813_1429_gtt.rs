 * include error handling, and maintain good documentation and maintainability.
 */

#[macro_use]
extern crate rocket;

// Import necessary modules from Rocket.
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::State;
use rocket::response::status;
use rocket::response::status::Custom;
use rocket::outcome::Outcome::{Success, Failure};

// Define a struct to hold test report data.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct TestReport {
    name: String,
    results: String,
    timestamp: String,
}

// Define an error type for report generation errors.
#[derive(Debug)]
enum ReportError {
    MissingData,
    InvalidFormat,
}

// Implement error handling for ReportError.
impl<'r> rocket::response::Responder<'r, 'static> for ReportError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            ReportError::MissingData => Err(Custom(Status::BadRequest, "Missing data for report generation").into()),
            ReportError::InvalidFormat => Err(Custom(Status::BadRequest, "Invalid report format").into()),
        }
    }
}

#[launch]
fn rocket() -> _ {
    // Initialize the Rocket server with a test report generator endpoint.
    rocket::build()
        .mount("/report", routes![generate_report])
        .manage(Json::new())
}

// Define a form struct to parse request data.
#[derive(FromForm)]
struct ReportForm<'r> {
    name: &'r str,
    results: &'r str,
}

// Define the function to generate test reports.
#[post("/report", format = "application/json", data = "<report_form>")]
fn generate_report(report_form: Form<ReportForm>) -> Result<Json<TestReport>, ReportError> {
    // Extract data from the form.
    let ReportForm { name, results } = report_form.into_inner();

    // Validate and process data.
    if name.is_empty() || results.is_empty() {
        return Err(ReportError::MissingData);
    }

    // Generate a timestamp for the report.
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Create a test report and return it as a JSON response.
    let report = TestReport {
        name: name.to_string(),
        results: results.to_string(),
        timestamp,
    };

    Ok(Json(report))
}
