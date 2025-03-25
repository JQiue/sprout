mod app;
mod components;
mod config;
mod helpers;
mod response;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  app::start().await
}
