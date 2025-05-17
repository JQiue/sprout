mod handler;
pub mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct SiteComponent;

impl SiteComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::create_site);
    cfg.service(handler::get_sites);
  }
}
