use actix_cors::Cors;
use actix_web::{
  middleware,
  web::{self, ServiceConfig},
  App, HttpResponse, HttpServer,
};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

use crate::{
  components::{agent::AgentComponent, deployment::DeploymentComponent, user::UserComponent},
  config::Config,
};

#[derive(Debug, Clone)]
pub struct AppState {
  pub db: DatabaseConnection,
}

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().json(serde_json::json!({"status": "OK"}))
}

pub fn config_app(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .configure(UserComponent::config)
      .configure(AgentComponent::config)
      .configure(DeploymentComponent::config)
      .route("/health", web::get().to(health_check)),
  );
}

pub async fn start() -> anyhow::Result<()> {
  let app_config = Config::from_env()?;
  let db = Database::connect(app_config.database_url).await.unwrap();
  match db.ping().await {
    Ok(_) => info!("Database is ok!"),
    Err(error) => panic!("{error}"),
  }
  let state = AppState { db };
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
