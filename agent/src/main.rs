mod app;
mod components;
mod config;
mod error;
mod helpers;
mod response;
mod rpc;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  app::start().await
}
