use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::{
    services::sites::{create_site, find_site_by_client_site},
    utils::sites_error::{self, SitesError},
};

#[derive(Debug, Deserialize, Validate)]
pub struct SitesRequest {
    #[validate(length(min = 1, message = "Customer name cannot be empty"))]
    pub customer_name: String,
    #[validate(length(min = 1, message = "Client site cannot be empty"))]
    pub client_site: String,
}
pub async fn add_site(
    db: web::Data<DatabaseConnection>,
    data: web::Json<SitesRequest>,
) -> Result<impl Responder, SitesError> {
    let site = data.into_inner();
    // Validate fields

    site.validate()?;
    match find_site_by_client_site(&db, site.client_site.clone()).await {
        Ok(_) => {
            return Err(SitesError::SiteValidateMessageError(
                "Site added error".to_string(),
            ));
        }
        Err(SitesError::SiteNotFoundError(_)) => {
            create_site(&db, site).await?;
            Ok(HttpResponse::Ok().body("Site added"))
        }
        Err(err) => Err(SitesError::SitesInternalServerError(err.to_string())),
    }
}
