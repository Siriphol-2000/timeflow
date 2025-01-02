use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait
};
use serde::Serialize;

use crate::{
    entities::{time_sheet, users,sites},
    error::APIError,
};
#[derive(FromQueryResult,Serialize)]

pub struct TestExport {
    pub id: i32,
    pub time_start: DateTimeWithTimeZone,
    pub time_end: DateTimeWithTimeZone,
    pub activity: String,
    #[sea_orm(column_type = "Float")]
    pub working_day: f32,
    #[sea_orm(column_type = "Float")]
    pub leaving_day: f32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub users_id: i32,
    pub sites_id: i32,
    pub name :String,
    pub last_name :String,
}


pub async fn find_timesheet_by_id(
    db: &DatabaseConnection,
    id: i32,
    sites_id: i32,
) -> Result<Vec<TestExport>, APIError> {
    let data = time_sheet::Entity::find()
        .column_as(users::Column::FirstName, "name")
        .column(users::Column::LastName)
        .join(JoinType::LeftJoin, users::Relation::TimeSheet.def().rev())
        .join(JoinType::LeftJoin, sites::Relation::TimeSheet.def().rev())
        .filter(time_sheet::Column::UsersId.eq(id)) // Filter by user ID
        .filter(time_sheet::Column::SitesId.eq(sites_id))
        .into_model::<TestExport>() // Filter by site ID
        .all(db).await.map_err(|e|{
            log::error!("Database error: {:?}", e);
            APIError::DatabaseError(e.into())
        })?;
        if data.is_empty(){
            APIError::NotFound("No data".into());
        }
        Ok(data)
}
