mod handler;
pub mod model;
mod service;

use actix_web::web::ServiceConfig;

pub struct UserComponent;

impl UserComponent {
  pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(handler::generate_casual_user);
    cfg.service(handler::user_register);
    cfg.service(handler::user_login);
    cfg.service(handler::get_user_info);
    cfg.service(handler::set_user_password);
    cfg.service(handler::refresh_user_token);
  }
}
