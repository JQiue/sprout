use std::net::IpAddr;

use actix_cors::Cors;
use actix_web::{
  App, HttpServer, middleware,
  web::{self, ServiceConfig},
};

use crate::{
  components::{
    base::health_check, deployment::DeploymentComponent, heartbeat::HeartbeatComponent,
  },
  config::Config,
  error::AppError,
};

#[derive(Debug, Clone)]
pub struct AppState {
  pub storage_path: String,
  pub nginx_config_path: String,
  pub upload_token_key: String,
  pub upload_token_key_expire: i64,
  pub public_ip: IpAddr,
}

pub fn config_app(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .configure(HeartbeatComponent::config)
      .configure(DeploymentComponent::config)
      .route("/health", web::get().to(health_check)),
  );
}

pub async fn start() -> Result<(), AppError> {
  let Config {
    host,
    port,
    workers,
    storage_path,
    nginx_config_path,
    upload_token_key,
    upload_token_key_expire,
    public_ip,
    ..
  } = Config::from_env()?;
  let state = AppState {
    storage_path,
    nginx_config_path,
    upload_token_key,
    upload_token_key_expire,
    public_ip,
  };
  Ok(
    HttpServer::new(move || {
      App::new()
        .app_data(web::Data::new(state.clone()))
        .wrap(Cors::permissive())
        .wrap(middleware::Logger::default())
        .configure(config_app)
    })
    .bind((host, port))?
    .workers(workers)
    .run()
    .await?,
  )
}
