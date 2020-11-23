use actix_web::{web};

use super::playbook;

fn config_api_playbook(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/new_entry")
        .route(web::post().to(playbook::create::handle))
    )
    .service(
        web::resource("/list")
        .route(web::get().to(playbook::list::handle))
    );
}

fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/playbook").configure(config_api_playbook)
    );
}

// this function could be located in different module
pub fn config_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").configure(config_api)
    );
}