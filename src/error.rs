use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use derive_more::derive::{Display, From};
use sea_orm::DbErr;
use validator::ValidationErrors;

#[derive(Debug, Display, From)]
pub enum APIError {
    #[display("DB error !!!")]
    DatabaseError(DbErr),
    #[display("Not Found !!!")]
    NotFound(String),
    #[display("Internal Server Error !!!")]
    InternalServerError(String),
    #[display("Validate Message Error !!!")]
    ValidateMessageError(String),
    #[display("Validate Error !!!")]
    #[from]
    ValidateError(ValidationErrors),
}
impl From<DbErr> for APIError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(message) => APIError::NotFound(message),
            _ => APIError::DatabaseError(err),
        }
    }
}
impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            APIError::DatabaseError(err) => HttpResponse::InternalServerError().json(ErrResponse {
                error: "DatabaseError".to_string(),
                message: format!("Database error: {}", err),
            }),
            APIError::NotFound(_) => HttpResponse::InternalServerError().json(ErrResponse {
                error: "Not Found".to_string(),
                message: format!("Not Found"),
            }),
            APIError::InternalServerError(err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "Internal ".to_string(),
                    message: format!("Internal Server error: {}", err),
                })
            }
            APIError::ValidateMessageError(err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "Validate error ".to_string(),
                    message: format!("Validate error: {}", err),
                })
            }
            APIError::ValidateError(err) => HttpResponse::InternalServerError().json(ErrResponse {
                error: "ValidateError".to_string(),
                message: format!("ValidateError error: {}", err),
            }),
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::DatabaseError(_)
            | APIError::NotFound(_)
            | APIError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::ValidateError(_) | APIError::ValidateMessageError(_) => {
                StatusCode::BAD_REQUEST
            }
        }
    }
}
#[derive(serde::Serialize)]
pub struct ErrResponse {
    error: String,
    message: String,
}
