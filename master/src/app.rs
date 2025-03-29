use std::net::Ipv4Addr;

use actix_cors::Cors;
use actix_web::{
  App, HttpServer, middleware,
  web::{self, ServiceConfig},
};
use sea_orm::DatabaseConnection;

use crate::{
  components::{
    agent::AgentComponent, base, deployment::DeploymentComponent, site::SiteComponent,
    user::UserComponent,
  },
  config::Config,
  error::AppError,
  migration::migrate,
};

#[derive(Debug, Clone)]
pub struct AppState {
  pub db: DatabaseConnection,
  pub login_token_key: String,
  pub register_agent_key: String,
  pub register_agent_key_expire: i64,
}

pub fn config_app(cfg: &mut ServiceConfig) {
  cfg.service(
    web::scope("/api")
      .configure(UserComponent::config)
      .configure(AgentComponent::config)
      .configure(SiteComponent::config)
      .configure(DeploymentComponent::config)
      .route("/health", web::get().to(base::health_check)),
  );
}

pub async fn start() -> Result<(), AppError> {
  let Config {
    workers,
    host,
    port,
    database_url,
    login_token_key,
    register_agent_key,
    register_agent_key_expire,
    ..
  } = Config::from_env()?;
  let db = migrate(&database_url).await?;
  db.ping().await?;
  let state = AppState {
    db,
    login_token_key: login_token_key,
    register_agent_key: register_agent_key,
    register_agent_key_expire: register_agent_key_expire,
  };
  Ok(
    HttpServer::new(move || {
      App::new()
        // .wrap(HttpAuthentication::with_fn(validator))
        .wrap(middleware::Logger::default())
        .wrap(Cors::permissive())
        .app_data(web::Data::new(state.clone()))
        .configure(config_app)
    })
    .bind((host, port))?
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .workers(workers)
    .run()
    .await?,
  )
}
