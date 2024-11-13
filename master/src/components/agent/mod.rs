mod handler;
mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct AgentComponent {}

impl AgentComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::register_server);
  }
}
