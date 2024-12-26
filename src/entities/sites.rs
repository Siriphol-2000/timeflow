//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sites")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub customer_name: String,
    pub client_site: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::time_sheet::Entity")]
    TimeSheet,
    #[sea_orm(has_many = "super::users_sites::Entity")]
    UsersSites,
}

impl Related<super::time_sheet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimeSheet.def()
    }
}

impl Related<super::users_sites::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersSites.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
