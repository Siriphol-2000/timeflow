use std::default;

use chrono::{FixedOffset, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    entities::time_sheet::{self, ActiveModel},
    utils::time_sheet_error::TimeSheetError,
};

pub struct TimeSheetRequest {
    time_start: chrono::DateTime<FixedOffset>,
    time_end: chrono::DateTime<FixedOffset>,
    activity: String,
    working_day: f32,
    leaving_day: f32,
    users_id: i32,
    sites_id: i32,
}

pub async fn create_time_sheet(
    db: DatabaseConnection,
    request: TimeSheetRequest,
) -> Result<time_sheet::Model, TimeSheetError> {
    let new_time_sheet: ActiveModel = ActiveModel {
        time_start: ActiveValue::set(request.time_start),
        time_end: sea_orm::ActiveValue::Set(request.time_end),
        activity: ActiveValue::set(request.activity),
        working_day: ActiveValue::set(request.working_day),
        leaving_day: ActiveValue::set(request.leaving_day),
        created_at: ActiveValue::set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        users_id: Set(request.users_id),
        sites_id: Set(request.sites_id),
        ..Default::default()
    };
    new_time_sheet.insert(&db).await.map_err(|e| {
        log::error!("Database error: {:?}", e);
        TimeSheetError::TimeSheetDatabaseError(e.into())
    })
}
pub async fn find_time_sheet_by_ids(
    db: DatabaseConnection,
    user_id: i32,
    sites_id: i32,
) -> Result<time_sheet::Model, TimeSheetError> {
    time_sheet::Entity::find()
        .filter(time_sheet::Column::UsersId.eq(user_id))
        .filter(time_sheet::Column::SitesId.eq(sites_id))
        .one(&db)
        .await?
        .ok_or_else(|| TimeSheetError::TimeSheetNotFoundError("time sheet already exist".into()))
}
