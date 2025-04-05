mod handler;
pub mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct DeploymentComponent;

impl DeploymentComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::create_deployment);
    cfg.service(handler::get_deployment_info);
    cfg.service(handler::update_deployment_status);
  }
}
