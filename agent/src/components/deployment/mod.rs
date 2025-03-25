use actix_web::web::ServiceConfig;

mod handler;
mod model;
mod service;
pub struct DeploymentComponent;

impl DeploymentComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::init_upload);
    cfg.service(handler::file_upload);
  }
}
