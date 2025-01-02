use actix_web::web;

use crate::handler::sites::add_site;

pub fn config_sites(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("api/v1/site").route("/create", web::post().to(add_site)));
}
