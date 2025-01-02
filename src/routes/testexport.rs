use actix_web::web;

use crate::handler::testexport::timsheet_export;

pub fn config_export(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/timesheet").route("/export", web::post().to(timsheet_export)));
}