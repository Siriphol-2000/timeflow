use chrono::{Duration, FixedOffset, NaiveDate};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection,
    EntityTrait, FromQueryResult, JoinType, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entities::{
        sites,
        time_sheet::{self, ActiveModel},
        users,
    },
    error::APIError,
    utils::time_sheet_error::TimeSheetError,
};

#[derive(Validate, Deserialize, Debug)]
pub struct TimeSheetRequest {
    pub time_start: chrono::DateTime<FixedOffset>,
    pub time_end: chrono::DateTime<FixedOffset>,
    #[validate(length(min = 1, message = "Customer name cannot be empty"))]
    pub activity: String,
    pub working_day: f32,
    pub leaving_day: f32,
    pub users_id: i32,
    pub sites_id: i32,
}

pub async fn create_time_sheet(
    db: &DatabaseConnection,
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
    new_time_sheet.insert(db).await.map_err(|e| {
        log::error!("Database error: {:?}", e);
        TimeSheetError::TimeSheetDatabaseError(e.into())
    })
}
pub async fn find_time_sheet_by_ids(
    db: &DatabaseConnection,
    user_id: i32,
    sites_id: i32,
) -> Result<time_sheet::Model, TimeSheetError> {
    time_sheet::Entity::find()
        .filter(time_sheet::Column::UsersId.eq(user_id))
        .filter(time_sheet::Column::SitesId.eq(sites_id))
        .one(db)
        .await?
        .ok_or_else(|| TimeSheetError::TimeSheetNotFoundError("".into()))
}

#[derive(FromQueryResult, Serialize)]

pub struct TestExport {
    pub id: i32,
    pub time_start: DateTimeWithTimeZone,
    pub time_end: DateTimeWithTimeZone,
    pub activity: String,
    #[sea_orm(column_type = "Float")]
    pub working_day: f32,
    #[sea_orm(column_type = "Float")]
    pub leaving_day: f32,
    pub first_name: String,
    pub last_name: String,
    pub customer_name: String,
    pub client_site: String,
}

pub async fn find_timesheet_by_id(
    db: &DatabaseConnection,
    id: i32,
    sites_id: i32,
    datetime: String,
) -> Result<Vec<TestExport>, APIError> {
    let v: Vec<&str> = datetime.split("/").collect();
    let mut yyyy: i32 = v[0]
        .parse()
        .map_err(|_| APIError::ValidateMessageError("Invalid year format".into()))?;
    let mut mm: u32 = v[1]
        .parse()
        .map_err(|_| APIError::ValidateMessageError("Invalid month format".into()))?;
    if mm > 12 || mm < 1 {
        return Err(APIError::ValidateMessageError(
            "Invalid month format".into(),
        ));
    }
    let dd: u32 = 1;
    let first_day = NaiveDate::from_ymd_opt(yyyy, mm, dd).expect("ไม่สามารถสร้างวันที่ค้นหาได้");

    if mm + 1 > 12 {
        mm = 1;
        yyyy = yyyy + 1;
    } else {
        mm = mm + 1;
    }
    let last_day = NaiveDate::from_ymd_opt(yyyy, mm, dd).expect("ไม่สามารถสร้างวันที่ค้นหาได้");
    let last_day_of_month = last_day - Duration::days(1);
    let data = time_sheet::Entity::find()
        .column(users::Column::FirstName)
        .column(users::Column::LastName)
        .column(sites::Column::CustomerName)
        .column(sites::Column::ClientSite)
        .join(JoinType::LeftJoin, users::Relation::TimeSheet.def().rev())
        .join(JoinType::LeftJoin, sites::Relation::TimeSheet.def().rev())
        .filter(time_sheet::Column::UsersId.eq(id))
        .filter(time_sheet::Column::SitesId.eq(sites_id))
        .filter(time_sheet::Column::TimeStart.gt(first_day))
        .filter(time_sheet::Column::TimeEnd.lt(last_day_of_month))
        .order_by_asc(time_sheet::Column::TimeStart)
        .into_model::<TestExport>()
        .all(db)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            APIError::DatabaseError(e.into())
        })?;
    if data.is_empty() {
        APIError::NotFound("No data".into());
    }
    Ok(data)
}
