mod handler;
pub mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct UserComponent;

impl UserComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::user_register);
    cfg.service(handler::user_login);
  }
}
