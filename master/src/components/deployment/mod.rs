mod handler;
mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct DeploymentComponent;

impl DeploymentComponent {
  pub fn config(cfg: &mut ServiceConfig) {}
}
