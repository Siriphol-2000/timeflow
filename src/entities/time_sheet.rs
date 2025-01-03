//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "time_sheet")]
pub struct Model {
    #[sea_orm(primary_key)]
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sites::Entity",
        from = "Column::SitesId",
        to = "super::sites::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Sites,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UsersId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::sites::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sites.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
