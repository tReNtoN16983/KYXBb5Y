#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use excel_writer::{Excel, ExcelFile};

// 定义一个结构体，用于序列化和反序列化前端请求的数据
#[derive(Serialize, Deserialize)]
struct GenerateExcelRequest {
    rows: Vec<Vec<String>>,
}

// 定义一个结构体，用于返回前端响应的数据
#[derive(Serialize, Deserialize)]
struct GenerateExcelResponse {
    message: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![generate_excel])
        .manage(ExcelFile::new())
}

// 定义一个路由，用于生成Excel文件
#[post("/generate_excel")]
fn generate_excel(request: Json<GenerateExcelRequest>, excel_file: &State<ExcelFile>) -> Json<GenerateExcelResponse> {
    // 从请求中获取数据
    let rows = request.into_inner().rows;

    // 创建一个Excel文件
    let mut excel = Excel::new();
    let sheet = excel.add_sheet("Sheet1");

    // 添加数据到Excel文件
    for row in rows {
        sheet.add_row();
        for cell in row {
            sheet.add_cell(cell);
        }
    }

    // 保存Excel文件
    let file_name = "generated_excel.xlsx";
    let path = Path::new(file_name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directory");
    }
    match excel_file.save_to_file(file_name) {
        Ok(_) => {
            Json(GenerateExcelResponse {
                message: "Excel file generated successfully".to_string(),
            })
        }
        Err(e) => {
            Json(GenerateExcelResponse {
                message: format!("Failed to generate Excel file: {}", e),
            })
        }
    }
}
