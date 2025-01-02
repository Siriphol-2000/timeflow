
use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::{error::APIError, services::testexport::find_timesheet_by_id};
#[derive(Debug, Deserialize, Validate)]
pub struct Timesheet{
#[validate(range(min = 1))]
 id : i32,
 sites_id : i32,
 //datetime  : String,
}
pub async fn timsheet_export(
    db: web::Data<DatabaseConnection>,
    data: web::Json<Timesheet>,
) -> Result<impl Responder, APIError> {
    let data = data.into_inner();
    println!("check_data : {}",data.id);
    let time_sheet = find_timesheet_by_id(&db, data.id,data.sites_id).await?;
    Ok(HttpResponse::Ok().json(time_sheet))
}
