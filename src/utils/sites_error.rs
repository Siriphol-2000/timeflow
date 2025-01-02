use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::derive::{Display, From};
use sea_orm::{DbErr, RuntimeErr};
use validator::{ValidationError, ValidationErrors};

#[derive(Debug, Display, From)]
pub enum SitesError {
    SitesDatabaseError(DbErr),
    SitesDatabaseConnectionError(RuntimeErr),
    SitesInternalServerError(String),
    SitesQueryError(RuntimeErr),
    SiteNotFoundError(String),
    #[display{"validation error"}]
    #[from]
    SitesValidationError(ValidationErrors),
    SiteValidateMessageError(String),
}
impl From<DbErr> for SitesError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(msg) => SitesError::SiteNotFoundError(msg),
            DbErr::Query(msg) => SitesError::SitesQueryError(msg),
            DbErr::Conn(msg) => SitesError::SitesDatabaseConnectionError(msg),
            _ => SitesError::SitesDatabaseError(err),
        }
    }
}
impl ResponseError for SitesError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            SitesError::SiteNotFoundError(err) => HttpResponse::NotFound().json(ErrResponse {
                error: "Not found error ".to_string(),
                message: format!(":{}", err),
            }),
            SitesError::SitesDatabaseError(db_err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "Database error ".to_string(),
                    message: format!(":{}", db_err),
                })
            }
            SitesError::SitesDatabaseConnectionError(runtime_err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "Database Connection error ".to_string(),
                    message: format!(":{}", runtime_err),
                })
            }
            SitesError::SitesInternalServerError(err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "Internal server error ".to_string(),
                    message: format!(":{}", err),
                })
            }
            SitesError::SitesQueryError(runtime_err) => {
                HttpResponse::InternalServerError().json(ErrResponse {
                    error: "Query Error".to_string(),
                    message: format!(":{}", runtime_err),
                })
            }
            SitesError::SitesValidationError(validation_errors) => HttpResponse::NotAcceptable()
                .json(ErrResponse {
                    error: "Validation Error".to_string(),
                    message: format!(":{}", validation_errors),
                }),
            SitesError::SiteValidateMessageError(err) => {
                HttpResponse::NotAcceptable().json(ErrResponse {
                    error: "Validation Error".to_string(),
                    message: format!(":{}", err),
                })
            }
        }
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SitesError::SitesDatabaseError(db_err) => StatusCode::INTERNAL_SERVER_ERROR,
            SitesError::SitesDatabaseConnectionError(runtime_err) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            SitesError::SitesInternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SitesError::SitesQueryError(runtime_err) => StatusCode::INTERNAL_SERVER_ERROR,
            SitesError::SiteNotFoundError(_) => StatusCode::NOT_FOUND,
            SitesError::SitesValidationError(validation_errors) => StatusCode::NOT_ACCEPTABLE,
            SitesError::SiteValidateMessageError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
#[derive(serde::Serialize)]
pub struct ErrResponse {
    error: String,
    message: String,
}
