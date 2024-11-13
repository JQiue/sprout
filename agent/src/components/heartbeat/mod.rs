mod service;

use actix_web::web::ServiceConfig;

pub struct HeartbeatComponent;

impl HeartbeatComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(service::register);
    cfg.service(service::heartbeat);
  }
}
