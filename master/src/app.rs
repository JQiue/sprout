use actix_cors::Cors;
use actix_web::{
  middleware,
  web::{self, ServiceConfig},
  App, HttpResponse, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use sea_orm::{Database, DatabaseConnection};

use crate::{
  components::{
    agent::AgentComponent, deployment::DeploymentComponent, site::SiteComponent,
    user::UserComponent,
  },
  config::Config,
  middleware::auth::validator,
};

#[derive(Debug, Clone)]
pub struct AppState {
  pub db: DatabaseConnection,
}

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().json(serde_json::json!({
    "status": "OK",
  }))
}

pub fn config_app(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .configure(UserComponent::config)
      .configure(AgentComponent::config)
      .configure(SiteComponent::config)
      .configure(DeploymentComponent::config)
      .route("/health", web::get().to(health_check)),
  );
}

pub async fn start() -> anyhow::Result<()> {
  let app_config = Config::from_env()?;
  let db = Database::connect(app_config.database_url).await?;
  db.ping().await?;
  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(AppState { db: db.clone() }))
      .wrap(HttpAuthentication::with_fn(validator))
      .wrap(middleware::Logger::default())
      .wrap(Cors::permissive())
      .configure(config_app)
  })
  .bind((app_config.host, app_config.port))?
  .workers(app_config.workers)
  .run()
  .await
  .map_err(anyhow::Error::from)
}
