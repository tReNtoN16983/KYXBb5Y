use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use thiserror::Error;
use plotters::prelude::*;
use plotters::coord::types::RangedCoordIf;
use plotters::chart::ChartBuilder;
use plotters::chart::ChartContext;
use plotters::chart::Series;
use plotters::coord::Shift;
use plotters::style::IntoColor;
use plotters::DrawingArea;
use plotters::IntoDrawingArea;
use plotters::chart::*;
use plotters::style::ThemeStyle;
use std::path::Path;
use std::fs::File;
use plotters::bitmap::BitmapBackend;
use serde::Deserialize;

// Define a custom error type for our application
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ChartError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

// Define a struct to represent chart data
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartData {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub data: Vec<(f64, f64)>,
}

// Define a struct to represent the chart configuration
#[derive(Deserialize, Serialize, Debug)]
pub struct ChartConfig {
    pub chart_type: String,
    pub width: u32,
    pub height: u32,
    pub theme: Option<String>,
}

// Define a function to generate a chart based on the chart configuration
fn generate_chart(config: &ChartConfig, data: &ChartData) -> Result<(), Box<dyn Error>> {
    // Create a chart context with the specified width and height
    let root_area = BitMapBackend::new("output.png", (config.width, config.height)).into_drawing_area();
    let drawing_area = root_area;

    // Set the theme if specified
    let theme = if let Some(theme) = &config.theme {
        ThemeStyle::try_from(theme)?
    } else {
        ThemeStyle::Default
    };

    // Create a new chart builder with the specified theme
    let chart = ChartBuilder::on(&drawing_area)
        .caption(&data.title, ("sans-serif", 50))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged((0..data.data.len() as u32).map(|i| i as f64), 0..data.data.iter().map(|&y| y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;

    // Add the chart title and labels
    chart.configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .draw()?;

    // Add the data series to the chart
    chart.draw_series(LabelledSeries::new(
        data.data.iter().map(|&(x, y)| (x, y)),
        &into_label()
    ))?;

    // Draw the chart
    chart.setup_coord().configure(|c| {
        c
            .name(&data.x_label, "sans-serif")
            .unit(c.x_label_area_size(), &data.y_label)
            .range(0..data.data.len() as f64)
            .label_style(("sans-serif", 20).into_color())
    }).draw()?;

    Ok(())
}

// Define a Rocket route for generating a chart
#[get("/chart")]
fn chart_generation_route() -> Result<status::Custom<Json<ChartConfig>>, status::Custom<Json<ChartError>>> {
    // Create a new chart configuration with default values
    let chart_config = ChartConfig {
        chart_type: "line".to_string(),
        width: 800,
        height: 600,
        theme: None,
    };

    // Create a new chart data with default values
    let chart_data = ChartData {
        title: "Example Chart".to_string(),
        x_label: "X Axis".to_string(),
        y_label: "Y Axis".to_string(),
        data: vec![(1.0, 2.0), (2.0, 3.0), (3.0, 5.0), (4.0, 7.0)],
    };

    // Generate the chart
    match generate_chart(&chart_config, &chart_data) {
        Ok(_) => Ok(status::Custom(Status::Ok, Json(chart_config))),
        Err(e) => Err(status::Custom(Status::InternalServerError, Json(ChartError::InvalidData(e.to_string())))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![chart_generation_route])
}