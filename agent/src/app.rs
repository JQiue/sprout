use actix_cors::Cors;
use actix_web::{
  App, HttpResponse, HttpServer, middleware,
  web::{self, ServiceConfig},
};

use crate::{
  components::{deployment::DeploymentComponent, heartbeat::HeartbeatComponent},
  config::Config,
  error::AppError,
};

#[derive(Debug, Clone)]
pub struct AppState {
  pub agent_token: String,
  pub master_rpc: rpc::Master::Rpc,
  pub storage_path: String,
  pub upload_token_key: String,
  pub upload_token_key_expire: i64,
}

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().json(serde_json::json!({"status": "OK"}))
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
  let app_config = Config::from_env()?;
  let state = AppState {
    master_rpc: rpc::Master::Rpc::new(app_config.master_url),
    agent_token: app_config.agent_token,
    storage_path: app_config.storage_path,
    upload_token_key: app_config.upload_token_key,
    upload_token_key_expire: app_config.upload_token_key_expire,
  };
  Ok(
    HttpServer::new(move || {
      App::new()
        .app_data(web::Data::new(state.clone()))
        .wrap(Cors::permissive())
        .wrap(middleware::Logger::default())
        .configure(config_app)
    })
    .bind((app_config.host, app_config.port))?
    .workers(app_config.workers)
    .run()
    .await?,
  )
}
