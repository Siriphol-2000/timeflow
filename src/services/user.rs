use crate::{
    entities::users::{self, ActiveModel},
    error::APIError,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
pub async fn create_user(
    db: &DatabaseConnection,
    first_name: String,
    last_name: String,
    role: String,
    email: String,
) -> Result<users::Model, APIError> {
    let new_user: ActiveModel = ActiveModel {
        first_name: Set(first_name),
        last_name: Set(last_name),
        role: Set(role),
        email: Set(email),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };
    // Insert the new user into the database
    new_user.insert(db).await.map_err(|e| {
        log::error!("Database error: {:?}", e);
        APIError::DatabaseError(e.into())
    })
}

pub async fn find_user_by_email(
    db: &DatabaseConnection,
    email: String,
) -> Result<users::Model, APIError> {
    users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await?
        .ok_or_else(|| APIError::NotFound("Eamil used".into()))
}
pub async fn find_user_by_id(db: &DatabaseConnection, id: i32) -> Result<users::Model, APIError> {
    users::Entity::find()
        .filter(users::Column::Id.eq(id))
        .one(db)
        .await?
        .ok_or_else(|| APIError::NotFound("".into()))
}
