use actix_web::web;

use crate::handler::time_sheet::create_time_sheet;

pub fn config_time_sheet(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/time-sheet").route("/create", web::post().to(create_time_sheet)),
    );
}
