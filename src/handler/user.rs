use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::APIError,
    services::user::{create_user, find_user_by_email},
};

#[derive(Debug, Deserialize, Validate)]
pub struct Register {
    #[validate(length(min = 1, message = "First name cannot be empty"))]
    first_name: String,
    #[validate(length(min = 1, message = "Last name cannot be empty"))]
    last_name: String,
    #[validate(length(min = 1, message = "Role cannot be empty"))]
    role: String,
    #[validate(email)]
    email: String,
}
pub async fn register(
    db: web::Data<DatabaseConnection>,
    data: web::Json<Register>,
) -> Result<impl Responder, APIError> {
    let user = data.into_inner();
    // Validate fields
    user.validate()?;
    match find_user_by_email(&db, user.email.clone()).await {
        Ok(_) => {
            return Err(APIError::ValidateMessageError(
                "Email is already registered".to_string(),
            ));
        }
        Err(APIError::NotFound(_)) => {
            create_user(&db, user.first_name, user.last_name, user.role, user.email).await?;
            Ok(HttpResponse::Ok().body("User registered"))
        }
        Err(err) => Err(APIError::InternalServerError(err.to_string())),
    }
}
