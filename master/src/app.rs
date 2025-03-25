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
  pub login_token_key: String,
  pub register_agent_key: String,
  pub register_agent_key_expire: i64,
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
  let state = AppState {
    db,
    login_token_key: app_config.login_token_key,
    register_agent_key: app_config.register_agent_key,
    register_agent_key_expire: app_config.register_agent_key_expire,
  };
  HttpServer::new(move || {
    App::new()
      .wrap(HttpAuthentication::with_fn(validator))
      .wrap(middleware::Logger::default())
      .wrap(Cors::permissive())
      .app_data(web::Data::new(state.clone()))
      .configure(config_app)
  })
  .bind((app_config.host, app_config.port))?
  .bind(("localhost", app_config.port))?
  .bind(("192.168.5.13", app_config.port))?
  .workers(app_config.workers)
  .run()
  .await
  .map_err(|error| {
    tracing::error!("{}", error);
    anyhow::Error::from(error)
  })
}
