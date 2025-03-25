mod handler;
pub mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct AgentComponent;

impl AgentComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::register_agent);
    cfg.service(handler::get_agent_status);
    cfg.service(handler::refresh_agent_token);
  }
}
