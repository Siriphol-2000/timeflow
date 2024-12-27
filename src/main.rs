use std::env;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use routes::user::config_user;
use sea_orm::Database;

mod entities;
mod error;
mod handler;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let db_connection = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_connection)
        .await
        .expect("DB connection failed, DATABASE_URL might be wrong");
    HttpServer::new(move || App::new().app_data(web::Data::new(db.clone())).configure(config_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
   //Ok(())
}
