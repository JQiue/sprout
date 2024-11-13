use actix_cors::Cors;
use actix_web::{
  middleware,
  web::{self, ServiceConfig},
  App, HttpResponse, HttpServer,
};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
  server_id: u8,
  storage_path: String,
}

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().json(serde_json::json!({"status": "OK"}))
}

pub fn config_app(cfg: &mut ServiceConfig) {
  cfg.service(web::scope("/api").route("/health", web::get().to(health_check)));
}

pub async fn start() -> anyhow::Result<()> {
  let app_config = Config::from_env()?;
  let state = AppState {
    server_id: app_config.server_id,
    storage_path: app_config.storage_path,
  };
  HttpServer::new(move || {
    let cors = Cors::permissive();
    App::new()
      .app_data(web::Data::new(state.clone()))
      .wrap(cors)
      .wrap(middleware::Logger::default())
      .configure(config_app)
  })
  .bind((app_config.host, app_config.port))?
  .workers(app_config.workers)
  .run()
  .await
  .map_err(anyhow::Error::from)
}
