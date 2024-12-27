use actix_web::web;

use crate::handler::user::register;


pub fn config_user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/user")
            .route("/add", web::post().to(register))
    );
}