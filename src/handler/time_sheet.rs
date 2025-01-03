use crate::{
    error::APIError, services::{sites::find_site_by_id, time_sheet::{create_time_sheet as create, find_time_sheet_by_ids, find_timesheet_by_id, TimeSheetRequest}}, utils::time_sheet_error::TimeSheetError
};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

pub async fn create_time_sheet(
    db: web::Data<DatabaseConnection>,
    data: web::Json<TimeSheetRequest>,
) -> Result<impl Responder, TimeSheetError> {
    let time_sheet = data.into_inner();
    time_sheet.validate()?;
    find_site_by_id(&db, time_sheet.sites_id).await.map_err(|_|{
        TimeSheetError::TimeSheetValidateMessageError(
            "User with the given ID does not exist".to_string(),
        )
    })?;

    match find_time_sheet_by_ids(&db, time_sheet.users_id, time_sheet.sites_id).await {
        Ok(_) => {
            return Err(TimeSheetError::TimeSheetValidateMessageError(
                "time sheet already created".to_string(),
            ));
        }
        Err(TimeSheetError::TimeSheetNotFoundError(_)) => {
            create(&db, time_sheet).await?;
            Ok(HttpResponse::Ok().body("time sheet created"))
        }
        Err(err) => Err(TimeSheetError::TimeSheetInternalServerError(
            err.to_string(),
        )),
    }
}
#[derive(Deserialize)]
pub struct Timesheet{
     id : i32,
     sites_id : i32,
     date_time : String,
    }
pub async fn export_time_sheet(
        db: web::Data<DatabaseConnection>,
        data: web::Json<Timesheet>,
    ) -> Result<impl Responder, APIError> {
        let data = data.into_inner();
        println!("check_data : {}",data.id);
        let time_sheet = find_timesheet_by_id(&db, data.id,data.sites_id,data.date_time).await?;
        Ok(HttpResponse::Ok().json(time_sheet))
    }