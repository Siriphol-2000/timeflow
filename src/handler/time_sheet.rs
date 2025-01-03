use std::{fs, path::Path};

use crate::{
    error::APIError,
    services::{
        sites::find_site_by_id,
        time_sheet::{
            create_time_sheet as create, find_time_sheet_by_ids, find_timesheet_by_id, TestExport, TimeSheetRequest
        },
    },
    utils::time_sheet_error::TimeSheetError,
};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;
use genpdf::{elements::{Break, Paragraph}, fonts, Alignment, Document, SimplePageDecorator};

/// Handler to create a new timesheet
pub async fn create_time_sheet(
    db: web::Data<DatabaseConnection>,
    data: web::Json<TimeSheetRequest>,
) -> Result<impl Responder, TimeSheetError> {
    // Validate the input data
    let time_sheet = data.into_inner();
    time_sheet.validate()?;

    // Check if the site exists
    find_site_by_id(&db, time_sheet.sites_id)
        .await
        .map_err(|_| {
            TimeSheetError::TimeSheetValidateMessageError(
                "Site with the given ID does not exist".to_string(),
            )
        })?;

    // Check if a timesheet already exists for the user and site
    match find_time_sheet_by_ids(&db, time_sheet.users_id, time_sheet.sites_id).await {
        Ok(_) => {
            Err(TimeSheetError::TimeSheetValidateMessageError(
                "Timesheet already created".to_string(),
            ))
        }
        Err(TimeSheetError::TimeSheetNotFoundError(_)) => {
            // Create a new timesheet if none exists
            create(&db, time_sheet).await?;
            Ok(HttpResponse::Ok().body("Timesheet created"))
        }
        Err(err) => Err(TimeSheetError::TimeSheetInternalServerError(
            err.to_string(),
        )),
    }
}

/// Struct to represent a timesheet for export
#[derive(Deserialize)]
pub struct Timesheet {
    id: i32,
    sites_id: i32,
    date_time: String,
}

/// Handler to export a timesheet
pub async fn export_time_sheet(
    db: web::Data<DatabaseConnection>,
    data: web::Json<Timesheet>,
) -> Result<impl Responder, APIError> {
    // Extract the input data
    let data = data.into_inner();
    println!("Exporting Timesheet ID: {}", data.id);

    // Fetch the timesheet data from the database
    let time_sheet = find_timesheet_by_id(&db, data.id, data.sites_id, data.date_time).await?;

    // let dir_path = "./src/utils/Open_Sans"; // Change this to your directory path
    // if let Err(e) = list_files_in_directory(dir_path) {
    //     eprintln!("Error listing files: {}", e);
    // }

    // Generate the PDF and return as a response
    let pdf = generate_pdf(&time_sheet)?;
    Ok(HttpResponse::Ok()
        .content_type("application/pdf")
        .body(pdf))
}

fn generate_pdf(timesheets: &Vec<TestExport>) -> Result<(), APIError> {
   // Load font family from the directory, not the file path directly
let font_family = fonts::from_files("./src/utils/", "Caladea", None)
.expect("Failed to load font family");

    // Create a document and set the default font family
    let mut doc = Document::new(font_family);
    doc.set_title("Timesheet Report");

    // Customize the page margins
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Add the content (timesheet data) for each item in the vector
    doc.push(Paragraph::new("Timesheet Report"));
    doc.push(Break::new(1));  // Line break

    for timesheet in timesheets.iter() {
        doc.push(Paragraph::new(format!("ID: {}", timesheet.id)));
        doc.push(Paragraph::new(format!("First Name: {}", timesheet.first_name)));
        doc.push(Paragraph::new(format!("Last Name: {}", timesheet.last_name)));
        doc.push(Paragraph::new(format!("Activity: {}", timesheet.activity)));
        doc.push(Paragraph::new(format!("Customer: {}", timesheet.customer_name)));
        doc.push(Paragraph::new(format!("Client Site: {}", timesheet.client_site)));
        doc.push(Paragraph::new(format!("Start Time: {}", timesheet.time_start.to_string())));
        doc.push(Paragraph::new(format!("End Time: {}", timesheet.time_end.to_string())));
        doc.push(Paragraph::new(format!("Working Day (hours): {}", timesheet.working_day)));
        doc.push(Paragraph::new(format!("Leaving Day (hours): {}", timesheet.leaving_day)));
        doc.push(Break::new(1));  // Add a line break after each entry
    }

    // Render the document and write it to a file
    doc.render_to_file("output.pdf").expect("Failed to write PDF file");

    Ok(())
}

fn list_files_in_directory(dir_path: &str) -> std::io::Result<()> {
    let path = Path::new(dir_path);
    
    // Ensure the path is a valid directory
    if path.is_dir() {
        // Read the directory
        let entries = fs::read_dir(path)?;
        
        // Iterate through the entries and print the file names
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            
            // Print file path
            if entry_path.is_file() {
                println!("{}", entry_path.display());
            }
        }
    } else {
        println!("The path is not a directory.");
    }

    Ok(())
}