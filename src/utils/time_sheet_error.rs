use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::derive::{Display, From};
use sea_orm::{DbErr, RuntimeErr};
use validator::ValidationErrors;

use super::sites_error::ErrResponse;

#[derive(Debug, Display, From)]
pub enum TimeSheetError {
    TimeSheetDatabaseError(DbErr),
    TimeSheetDatabaseConnectionError(RuntimeErr),
    TimeSheetInternalServerError(String),
    TimeSheetQueryError(RuntimeErr),
    TimeSheetNotFoundError(String),
    #[display{"validation error"}]
    #[from]
    TimeSheetValidationError(ValidationErrors),
    TimeSheetValidateMessageError(String),
}

impl From<DbErr> for TimeSheetError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::Conn(runtime_err) => {
                TimeSheetError::TimeSheetDatabaseConnectionError(runtime_err)
            }
            DbErr::Query(runtime_err) => TimeSheetError::TimeSheetQueryError(runtime_err),
            DbErr::RecordNotFound(_) => {
                TimeSheetError::TimeSheetNotFoundError("record not found".to_string())
            }
            _ => TimeSheetError::TimeSheetDatabaseError(err),
        }
    }
}
impl ResponseError for TimeSheetError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            TimeSheetError::TimeSheetDatabaseError(db_err) => HttpResponse::InternalServerError()
                .json(ErrResponse {
                    error: "database error".to_string(),
                    message: db_err.to_string(),
                }),
            TimeSheetError::TimeSheetDatabaseConnectionError(runtime_err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "database connection error".to_string(),
                    message: runtime_err.to_string(),
                })
            }
            TimeSheetError::TimeSheetInternalServerError(err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "server error".to_string(),
                    message: err.to_string(),
                })
            }
            TimeSheetError::TimeSheetQueryError(runtime_err) => HttpResponse::InternalServerError()
                .json(ErrResponse {
                    error: "query error".to_string(),
                    message: runtime_err.to_string(),
                }),
            TimeSheetError::TimeSheetNotFoundError(err) => HttpResponse::InternalServerError()
                .json(ErrResponse {
                    error: "user not found".to_string(),
                    message: err.to_string(),
                }),
            TimeSheetError::TimeSheetValidationError(validation_errors) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "validation error".to_string(),
                    message: validation_errors.to_string(),
                })
            }
            TimeSheetError::TimeSheetValidateMessageError(err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "errer".to_string(),
                    message: err.to_string(),
                })
            }
        }
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            TimeSheetError::TimeSheetDatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TimeSheetError::TimeSheetDatabaseConnectionError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            TimeSheetError::TimeSheetInternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TimeSheetError::TimeSheetQueryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TimeSheetError::TimeSheetNotFoundError(_) => StatusCode::NOT_FOUND,
            TimeSheetError::TimeSheetValidationError(_) => StatusCode::NOT_ACCEPTABLE,
            TimeSheetError::TimeSheetValidateMessageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
