use std::result;

use sea_orm::DatabaseConnection;
use sea_orm::{sqlx::types::chrono, Set};

use crate::entities::sites::ActiveModel;
use crate::entities::sites;
use crate::utils::sites_error::SitesError;

struct SitesRequest{
    customer_name:String,
    client_site:String,
    created_at:chrono::DateTime<chrono::Utc>,
    updated_at:chrono::DateTime<chrono::Utc>,
}
// pub async fn create_site( db: &DatabaseConnection,site:SitesRequest)->Result<SitesRequest,SitesError>{
//     let new_site :ActiveModel = ActiveModel{
//         customer_name:Set(site.customer_name),
//         client_site: Set(site.client_site),
//         created_at:   Set(chrono::Utc::now().into()) ,
//         updated_at: Set(chrono::Utc::now().into()),
//     };
// }
// pub async fn create_user(
//     db: &DatabaseConnection,
    
// ) -> Result<users::Model, APIError> {
//     let new_user: ActiveModel = ActiveModel {
//         first_name: Set(first_name),
//         last_name: Set(last_name),
//         role: Set(role),
//         email: Set(email),
//         created_at: Set(chrono::Utc::now().into()),
//         updated_at: Set(chrono::Utc::now().into()),
//         ..Default::default()
//     };
//     // Insert the new user into the database
//     new_user.insert(db).await.map_err(|e| {
//         log::error!("Database error: {:?}", e);
//         APIError::DatabaseError(e.into())
//     })
// }