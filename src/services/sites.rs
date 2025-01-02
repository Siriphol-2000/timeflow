use std::result;

use crate::entities::sites::{self, ActiveModel};
use crate::handler::sites::SitesRequest;
use crate::utils::sites_error::{self, SitesError};
use sea_orm::{sqlx::types::chrono, Set};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn create_site(
    db: &DatabaseConnection,
    site: SitesRequest,
) -> Result<sites::Model, SitesError> {
    let new_site: ActiveModel = ActiveModel {
        customer_name: Set(site.customer_name),
        client_site: Set(site.client_site),
        created_at: Set(chrono::Utc::now().into()),
        updated_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    };
    let result = new_site.insert(db).await?;
    Ok(sites::Model {
        id: result.id,
        customer_name: result.customer_name,
        client_site: result.client_site,
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}
pub async fn find_site_by_client_site(
    db: &DatabaseConnection,
    site_name: String,
) -> Result<sites::Model, SitesError> {
    let result = sites::Entity::find()
        .filter(sites::Column::ClientSite.eq(site_name))
        .one(db)
        .await?
        .ok_or_else(|| SitesError::SiteNotFoundError("client site name used".into()));
    result
}
