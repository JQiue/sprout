use std::{net::Ipv4Addr, time::Duration};

use actix_cors::Cors;
use actix_web::{
  App, HttpServer, middleware,
  rt::time,
  web::{self, ServiceConfig},
};
use rpc::{AgentRpc, CloudflareRpc};

use crate::{
  components::{
    agent::AgentComponent, base, deployment::DeploymentComponent, site::SiteComponent,
    user::UserComponent,
  },
  config::Config,
  error::AppError,
  migration::migrate,
  repository::RepositoryManager,
  timing::scheduled_task,
};

#[derive(Debug, Clone)]
pub struct AppState {
  pub repo: RepositoryManager,
  pub login_token_key: String,
  pub register_agent_key: String,
  pub register_agent_key_expire: i64,
  pub agent_rpc: AgentRpc,
  pub cloudflare_rpc: rpc::CloudflareRpc,
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
    cloudflare_zone_id,
    cloudflare_api_key,
    cloudflare_email,
  } = Config::from_env()?;
  let db = migrate(&database_url).await?;
  db.ping().await?;
  let repo = RepositoryManager::new(db);
  let state = AppState {
    repo: repo.clone(),
    login_token_key,
    register_agent_key,
    register_agent_key_expire,
    agent_rpc: AgentRpc::new()?,
    cloudflare_rpc: CloudflareRpc::new(cloudflare_zone_id, cloudflare_email, cloudflare_api_key)
      .await?,
  };

  actix_web::rt::spawn(async move {
    let mut interval = time::interval(Duration::from_secs(5));
    loop {
      interval.tick().await;
      scheduled_task(&repo).await;
    }
  });

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
